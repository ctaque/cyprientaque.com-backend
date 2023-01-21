port module Elm.Components.Gallery exposing (..)

import Browser
import CssClass exposing (CssClass, class, classList)
import Elm.Decoders.Index
    exposing
        ( decodeImages
        , decodeProject
        , envDecoder
        , filterCategoryDecoder
        , jwtTokenDecoder
        )
import Html exposing (Html, a, button, div, h1, h2, h3, h4, img, nav, p, small, span, text)
import Html.Attributes exposing (attribute, style)
import Html.Events exposing (onClick, stopPropagationOn)
import Http
import Json.Decode as Decode exposing (Decoder, Value, bool, int, list, nullable, string)
import RemoteData exposing (WebData)
import Svg exposing (svg)
import Svg.Attributes
import Task
import Time exposing (posixToMillis, utc)
import Types.Gallery exposing (Msg(..))
import Types.Index
    exposing
        ( AccessToken
        , Author
        , BitbucketProject
        , Carousel(..)
        , Category
        , Env(..)
        , FilterCategory(..)
        , Image
        , ImgDimensions
        , ProfileImage
        , Project
        , Repository
        , RepositoryListResponse
        , WheelDirection(..)
        )


main : Program Flags Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


type alias Style =
    { gallery : CssClass
    }


type alias Flags =
    Value


type alias Model =
    { images : WebData (List Image)
    , env : Env
    , style : Style
    , jwtToken : String
    , fadeIndex : Int
    , activeImage : Maybe Image
    , activeProject : WebData Project
    , branch : FilterCategory
    }


cssDecoder : Decoder Style
cssDecoder =
    Decode.map Style
        (Decode.field "style" CssClass.decode)


fetchImages : Env -> String -> FilterCategory -> Cmd Msg
fetchImages env jwtToken branch =
    let
        baseUrl =
            case env of
                Production ->
                    "https://ctprods.cyprientaque.com/projectImage/includeExcludeProjectCategories"

                _ ->
                    "http://localhost:8088/projectImage/includeExcludeProjectCategories"
    in
    Http.request
        { url =
            case branch of
                Web ->
                    baseUrl ++ "?include_categories=[1]&exclude_categories=[]"

                Other ->
                    baseUrl ++ "?include_categories=[]&exclude_categories=[1]"
        , expect = Http.expectJson (RemoteData.fromResult >> GetImagesResponse) decodeImages
        , method = "GET"
        , timeout = Nothing
        , tracker = Nothing
        , body = Http.emptyBody
        , headers = [ Http.header "Authorization" ("Bearer " ++ jwtToken) ]
        }


addImageView : Int -> Env -> String -> Cmd Msg
addImageView projectImageId env jwtToken =
    Http.request
        { url =
            case env of
                Production ->
                    "https://ctprods.cyprientaque.com/projectImage/" ++ String.fromInt projectImageId ++ "/addView"

                _ ->
                    "http://localhost:8088/projectImage/" ++ String.fromInt projectImageId ++ "/addView"
        , expect = Http.expectString AddImageViewResponse
        , method = "PUT"
        , timeout = Nothing
        , tracker = Nothing
        , body = Http.emptyBody
        , headers = [ Http.header "Authorization" ("Bearer " ++ jwtToken) ]
        }


fetchProject : Int -> Env -> String -> Cmd Msg
fetchProject projectId env jwtToken =
    Http.request
        { url =
            case env of
                Production ->
                    "https://ctprods.cyprientaque.com/projects/" ++ String.fromInt projectId

                _ ->
                    "http://localhost:8088/projects/" ++ String.fromInt projectId
        , expect = Http.expectJson (RemoteData.fromResult >> GetProjectResponse) decodeProject
        , method = "GET"
        , timeout = Nothing
        , tracker = Nothing
        , body = Http.emptyBody
        , headers = [ Http.header "Authorization" ("Bearer " ++ jwtToken) ]
        }


init : Flags -> ( Model, Cmd Msg )
init f =
    let
        style =
            Decode.decodeValue cssDecoder f
                |> Result.toMaybe
                |> Maybe.withDefault
                    { gallery = CssClass.empty }

        env =
            Decode.decodeValue envDecoder f
                |> Result.toMaybe
                |> Maybe.withDefault
                    Production

        jwtToken =
            Decode.decodeValue jwtTokenDecoder f
                |> Result.toMaybe
                |> Maybe.withDefault
                    ""

        branch =
            Decode.decodeValue filterCategoryDecoder f
                |> Result.toMaybe
                |> Maybe.withDefault
                    Web
    in
    ( Model
        RemoteData.Loading
        env
        style
        jwtToken
        0
        Maybe.Nothing
        RemoteData.NotAsked
        branch
    , Cmd.batch
        [ fetchImages env jwtToken branch
        ]
    )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        AddImageViewResponse resp ->
            ( model, Cmd.none )

        GotBranch branch ->
            let
                br =
                    case branch of
                        "software" ->
                            Web

                        _ ->
                            Other
            in
            ( { model | branch = br, images = RemoteData.Loading, fadeIndex = 0 }, fetchImages model.env model.jwtToken br )

        DoNothing ->
            ( model, Cmd.none )

        GetImagesResponse webdata ->
            ( { model | images = webdata }, Cmd.none )

        GotJwt jwt ->
            ( { model | jwtToken = jwt }, Cmd.none )

        FadeIn time ->
            let
                fadeIndex =
                    model.fadeIndex
            in
            ( { model | fadeIndex = fadeIndex + 1 }, Cmd.none )

        SetActiveImage image ->
            let
                cmd =
                    case image of
                        Just i ->
                            ( fetchProject i.project_id model.env model.jwtToken, addImageView i.id model.env model.jwtToken )

                        Nothing ->
                            ( Cmd.none, Cmd.none )
            in
            ( { model | activeImage = image, activeProject = RemoteData.NotAsked }, Cmd.batch [ Tuple.first cmd, Tuple.second cmd ] )

        GetProjectResponse webdata ->
            ( { model | activeProject = webdata }, Cmd.none )


mapImages : Style -> Int -> Int -> Image -> Html Msg
mapImages style fadeIndex index image =
    button
        [ classList style.gallery
            [ ( "listItem", True )
            , ( "fade"
              , if fadeIndex >= index then
                    False

                else
                    True
              )
            ]
        , onClick (SetActiveImage (Maybe.Just image))
        ]
        [ img [ attribute "src" image.w350_object_url ] []
        ]


loader : Model -> Html msg
loader model =
    div [ class model.style.gallery "loader-wrapper" ]
        [ div [ class model.style.gallery "lds-ring" ]
            [ div [] []
            , div [] []
            , div [] []
            ]
        ]


mapThumbnails : Style -> Int -> Image -> Html msg
mapThumbnails cssClass index image =
    div [ class cssClass.gallery "imageThumbnail" ] [ img [ attribute "src" image.w350_object_url ] [] ]


onLinkClick : msg -> Html.Attribute msg
onLinkClick msg =
    stopPropagationOn "click" (Decode.map alwaysStopPropagation (Decode.succeed msg))


alwaysStopPropagation : msg -> ( msg, Bool )
alwaysStopPropagation msg =
    ( msg, True )


view : Model -> Html Msg
view model =
    div []
        [ case model.activeImage of
            Just image ->
                div
                    [ class model.style.gallery "overlay"
                    , onClick (SetActiveImage Maybe.Nothing)
                    ]
                    [ div
                        [ class model.style.gallery "overlay--inner-wrapper"
                        , onLinkClick DoNothing
                        ]
                        [ case model.activeProject of
                            RemoteData.Success p ->
                                div []
                                    [ div [ class model.style.gallery "imageWrapper" ]
                                        [ img
                                            [ attribute "src" image.w1500_object_url
                                            ]
                                            []
                                        ]
                                    , div
                                        [ class model.style.gallery "content"
                                        ]
                                        [ div
                                            [ class model.style.gallery "projectDetail"
                                            ]
                                            [ div [ class model.style.gallery "thumbnailsWrapper" ]
                                                (p.images
                                                    |> List.take 4
                                                    |> List.indexedMap (mapThumbnails model.style)
                                                )
                                            , div [ class model.style.gallery "titleLinkWrapper" ]
                                                [ h2 [] [ text p.title ]
                                                , a [ attribute "href" ("/portfolio?p=" ++ p.slug ++ "&i=" ++ String.fromInt image.id) ]
                                                    [ span [] [ text "Voir le projet" ]
                                                    , span [ class model.style.gallery "rightArrow" ]
                                                        [ svg
                                                            [ Svg.Attributes.width "20"
                                                            , Svg.Attributes.height "20"
                                                            , Svg.Attributes.viewBox "0 0 24 24"
                                                            ]
                                                            [ Svg.path
                                                                [ Svg.Attributes.d "M0 0h24v24H0z"
                                                                , Svg.Attributes.fill "none"
                                                                ]
                                                                []
                                                            , Svg.path
                                                                [ Svg.Attributes.d "M20 11H7.83l5.59-5.59L12 4l-8 8 8 8 1.41-1.41L7.83 13H20v-2z"
                                                                , Svg.Attributes.fill "#66b3ff"
                                                                ]
                                                                []
                                                            ]
                                                        ]
                                                    ]
                                                ]
                                            ]
                                        ]
                                    ]

                            RemoteData.Loading ->
                                loader model

                            RemoteData.Failure e ->
                                div [] [ text (Debug.toString e) ]

                            RemoteData.NotAsked ->
                                div [] []
                        ]
                    ]

            Nothing ->
                div [] []
        , case model.images of
            RemoteData.Success images ->
                div [ class model.style.gallery "listWrapper" ] (List.indexedMap (mapImages model.style model.fadeIndex) images)

            RemoteData.Loading ->
                div [] [ loader model ]

            _ ->
                div [] []
        ]


port getJwt : (String -> msg) -> Sub msg


port getBranch : (String -> msg) -> Sub msg


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.batch
        [ getJwt GotJwt
        , case model.images of
            RemoteData.Success i ->
                case model.fadeIndex > List.length i of
                    True ->
                        Sub.none

                    False ->
                        case model.activeImage of
                            Just image ->
                                Sub.none

                            Nothing ->
                                Time.every 50 FadeIn

            _ ->
                Sub.none
        , getBranch GotBranch
        ]
