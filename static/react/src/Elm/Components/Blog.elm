port module Elm.Components.Blog exposing (..)

import Browser
import CssClass exposing (CssClass, class, classList)
import DateFormat exposing (english, format, formatI18n)
import Elm.Decoders.Index exposing (decodeProjects, envDecoder, jwtTokenDecoder)
import Elm.Lib.Functions exposing (errorToString)
import Filesize
import Html exposing (Html, a, button, div, h1, h3, img, nav, p, small, span, text)
import Html.Attributes exposing (attribute, href, style)
import Html.Events exposing (onClick)
import Http
import Json.Decode as Decode exposing (Decoder, Error, Value, bool, decodeString, int, list, string)
import Markdown
import RemoteData exposing (WebData)
import Svg exposing (svg)
import Svg.Attributes
import Task
import Time exposing (utc)
import Types.Blog exposing (Msg(..))
import Types.Index exposing (Env(..), FilterCategory(..), Image, Project)


type alias Style =
    { blog : CssClass
    }


type alias Model =
    { articles : WebData (List Project)
    , style : Style
    , currentArticle : Maybe Project
    , currentImage : Maybe Image
    , slug : Maybe String
    , zone : Time.Zone
    , jwtToken : String
    , env : Env
    }


main : Program Value Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


init : Value -> ( Model, Cmd Msg )
init f =
    let
        style =
            Decode.decodeValue cssDecoder f
                |> Result.toMaybe
                |> Maybe.withDefault
                    { blog = CssClass.empty }

        timeZoneCommand =
            Task.perform GetTimeZone Time.here

        slug =
            Decode.decodeValue slugDecoder f
                |> Result.toMaybe
                |> Maybe.withDefault Maybe.Nothing

        env =
            Decode.decodeValue envDecoder f
                |> Result.toMaybe
                |> Maybe.withDefault Production

        jwtToken =
            Decode.decodeValue jwtTokenDecoder f
                |> Result.toMaybe
                |> Maybe.withDefault ""
    in
    ( Model RemoteData.Loading style Maybe.Nothing Maybe.Nothing slug utc jwtToken env, fetchArticles env jwtToken )


fetchArticles : Env -> String -> Cmd Msg
fetchArticles env jwtToken =
    Http.request
        { url =
            case env of
                Production ->
                    "https://www.cyprientaque.com/projects/category/5"

                _ ->
                    "http://localhost:8088/projects/category/5"
        , expect = Http.expectJson (RemoteData.fromResult >> GetArticlesResponse) decodeProjects
        , method = "GET"
        , timeout = Nothing
        , tracker = Nothing
        , body = Http.emptyBody
        , headers = [ Http.header "Authorization" ("Bearer " ++ jwtToken) ]
        }


cssDecoder : Decoder Style
cssDecoder =
    Decode.map Style
        (Decode.field "style" CssClass.decode)


slugDecoder : Decoder (Maybe String)
slugDecoder =
    Decode.maybe (Decode.field "slug" string)


addView : Int -> Env -> String -> Cmd Msg
addView articletId env jwtToken =
    Http.request
        { url =
            case env of
                Production ->
                    "https://www.cyprientaque.com/projects/" ++ String.fromInt articletId ++ "/addView"

                _ ->
                    "http://localhost:8088/projects/" ++ String.fromInt articletId ++ "/addView"
        , expect = Http.expectString AddViewResponse
        , method = "PUT"
        , timeout = Nothing
        , tracker = Nothing
        , body = Http.emptyBody
        , headers = [ Http.header "Authorization" ("Bearer " ++ jwtToken) ]
        }


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        SetSlug project ->
            case project of
                Just p ->
                    ( model, setSlug (Maybe.Just p.slug) )

                Nothing ->
                    ( model, setSlug Maybe.Nothing )

        AddViewResponse res ->
            ( model, Cmd.none )

        GetTimeZone timezone ->
            ( { model | zone = timezone }, Cmd.none )

        GetArticlesResponse articles ->
            let
                currentArticle =
                    case articles of
                        RemoteData.Success gotArticles ->
                            case model.slug of
                                Just slug ->
                                    gotArticles
                                        |> List.filter (\article -> article.slug == slug)
                                        |> List.head

                                Nothing ->
                                    Maybe.Nothing

                        _ ->
                            Maybe.Nothing
            in
            ( { model | articles = articles, currentArticle = currentArticle }
            , case currentArticle of
                Just article ->
                    addView article.id model.env model.jwtToken

                Nothing ->
                    Cmd.none
            )

        GotJwt jwtToken ->
            ( { model | jwtToken = jwtToken }, Cmd.none )

        GotSlug slug ->
            let
                currentArticle =
                    case model.articles of
                        RemoteData.Success gotArticles ->
                            case slug of
                                Just s ->
                                    gotArticles
                                        |> List.filter (\article -> article.slug == s)
                                        |> List.head

                                Nothing ->
                                    Maybe.Nothing

                        _ ->
                            Maybe.Nothing

                addViewCmd =
                    case currentArticle of
                        Just a ->
                            addView a.id model.env model.jwtToken

                        Nothing ->
                            Cmd.none
            in
            ( { model | slug = slug, currentArticle = currentArticle }, addViewCmd )


mapArticlesTitles : Model -> Project -> Html Msg
mapArticlesTitles model article =
    let
        coverImage =
            getCoverImage article
    in
    div [ class model.style.blog "list-item", Html.Events.onClick (SetSlug (Maybe.Just article)) ]
        [ case coverImage of
            Just img ->
                div [ class model.style.blog "header" ] [ Html.img [ attribute "src" img.w350_object_url ] [] ]

            Nothing ->
                div [] []
        , small [ class model.style.blog "date" ]
            [ text (formatI18n english "MMMM dd yyyy" model.zone article.created_at)
            ]
        , a [ class model.style.blog "title", href ("/blog/" ++ article.slug) ]
            [ text article.title ]
        , p [ class model.style.blog "preview" ] [ text (String.replace "#" "" (String.slice 0 200 article.content ++ "...")) ]
        ]


getCoverImage : Project -> Maybe Image
getCoverImage project =
    let
        primaryImage =
            project.images
                |> List.filter .primary
                |> List.head
    in
    primaryImage


view : Model -> Html Msg
view model =
    case model.currentArticle of
        Just article ->
            div [ class model.style.blog "article" ]
                [ case getCoverImage article of
                    Nothing ->
                        div [] []

                    Just image ->
                        div [ class model.style.blog "header" ]
                            [ img [ class model.style.blog "banner", attribute "src" image.w1500_object_url ] []
                            , div [ class model.style.blog "gradient" ] []
                            , div [ class model.style.blog "page-title" ]
                                [ button [ Html.Events.onClick (SetSlug Maybe.Nothing), class model.style.blog "back-link" ]
                                    [ svg [ Svg.Attributes.width "40", Svg.Attributes.height "40", Svg.Attributes.viewBox "0 0 24 24" ]
                                        [ Svg.path [ Svg.Attributes.d "M0 0h24v24H0z", Svg.Attributes.fill "none" ] []
                                        , Svg.path [ Svg.Attributes.d "M16.01 11H4v2h12.01v3L20 12l-3.99-4z", Svg.Attributes.fill "rgb(169,169,171)" ] []
                                        ]
                                    ]
                                , div [ class model.style.blog "title" ]
                                    [ h1 [] [ text article.title ]
                                    , small [ class model.style.blog "date" ]
                                        [ text (formatI18n english "dddd, dd MMMM yyyy" model.zone article.created_at) ]
                                    ]
                                ]
                            ]
                , div []
                    [ Markdown.toHtml [ class model.style.blog "content" ] article.content
                    ]
                , div [ class model.style.blog "footer" ]
                    [ case article.updated_at of
                        Just date ->
                            span [ class model.style.blog "edition-date" ] [ text ("Last edited : " ++ formatI18n english "dddd, dd MMMM yyyy HH:mm" utc date) ]

                        Nothing ->
                            span [] []
                    , a
                        [ class model.style.blog "edit"
                        , attribute "href" "https://www.cyprientaque.com/login"
                        , attribute "target" "_blank"
                        ]
                        [ text "edit" ]
                    ]
                ]

        Nothing ->
            case model.articles of
                RemoteData.Failure error ->
                    div [] [ text (errorToString error) ]

                RemoteData.Success articles ->
                    div [ class model.style.blog "articles-list--with-banner" ]
                        [ div [ class model.style.blog "banner-wrapper" ]
                            [ img
                                [ class model.style.blog "banner"
                                , attribute "src" "https://s3-eu-west-1.amazonaws.com/ctaque.logos/mesquer_banner.jpg"
                                ]
                                []
                            ]
                        , div [ class model.style.blog "articles-list" ]
                            (articles
                                |> List.sortBy .id
                                |> List.reverse
                                |> List.map (mapArticlesTitles model)
                            )
                        ]

                _ ->
                    div [] []


port getJwt : (String -> msg) -> Sub msg


port setSlug : Maybe String -> Cmd msg


port getSlug : (Maybe String -> msg) -> Sub msg


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.batch [ getJwt GotJwt, getSlug GotSlug ]
