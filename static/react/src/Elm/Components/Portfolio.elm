port module Elm.Components.Portfolio exposing (..)

import Array
import Browser
import CssClass exposing (CssClass, class, classList)
import DateFormat exposing (formatI18n, french)
import Delay exposing (after)
import Dict exposing (Dict)
import Dict.Extra as DictExtra
import Elm.Decoders.Index
    exposing
        ( decodeAccessToken
        , decodeBitbucketProject
        , decodeBitbucketRepositoryResponse
        , decodeCategories
        , decodeIds
        , decodeImageLoaded
        , decodeProjects
        , envDecoder
        , filterCategoryDecoder
        , jwtTokenDecoder
        , langDecoder
        , slugDecoder
        )
import Elm.Lib.Functions exposing (errorToString, firstIndexOf)
import Html exposing (Html, a, button, div, h1, h2, h3, h4, img, input, nav, p, small, span, text)
import Html.Attributes exposing (attribute, style, value)
import Html.Events exposing (keyCode, on, onClick, onInput, stopPropagationOn)
import Html.Events.Extra.Wheel as Wheel
import Http
import Json.Decode as Decode exposing (Decoder, Value)
import Keyboard exposing (rawValue)
import List.Extra
import Markdown
import RemoteData exposing (WebData)
import Svg exposing (svg)
import Svg.Attributes
import Task
import Time exposing (posixToMillis, utc)
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
        , Lang(..)
        , ParentCategory
        , ProfileImage
        , Project
        , Repository
        , RepositoryListResponse
        , WheelDirection(..)
        )
import Types.Portfolio exposing (GetProjectsResponseNewData, Msg(..), OnMouseWheelNextData)


main : Program Flags Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


type alias Model =
    { projects : WebData (List Project)
    , style : Style
    , activeCategory : FilterCategory
    , currentProject : Maybe Project
    , currentImage : Maybe Image
    , imageLoaded : Maybe ImgDimensions
    , carousel : Carousel
    , env : Env
    , secSinceEpoch : Maybe Int
    , jwtToken : String
    , lastRepositoriesFetch : WebData RepositoryListResponse
    , repositoriesFullList : List Repository
    , accessToken : WebData AccessToken
    , bitbucketProject : WebData BitbucketProject
    , urlSlug : Maybe String
    , scrollTimer : Float
    , categories : WebData (List Category)
    , realCategory : Maybe Category
    , searchProjectsIds : WebData (List Int)
    , searchKeyword : String
    , lang : Lang
    }


type alias Flags =
    Value


type alias Style =
    { portfolio : CssClass
    }


cssDecoder : Decoder Style
cssDecoder =
    Decode.map Style
        (Decode.field "style" CssClass.decode)


init : Flags -> ( Model, Cmd Msg )
init f =
    let
        style =
            Decode.decodeValue cssDecoder f
                |> Result.toMaybe
                |> Maybe.withDefault
                    { portfolio = CssClass.empty }

        env =
            Decode.decodeValue envDecoder f
                |> Result.toMaybe
                |> Maybe.withDefault
                    Production

        activeCategory =
            Decode.decodeValue filterCategoryDecoder f
                |> Result.toMaybe
                |> Maybe.withDefault Web

        jwtToken =
            Decode.decodeValue jwtTokenDecoder f
                |> Result.toMaybe
                |> Maybe.withDefault
                    ""

        projectSlug =
            Decode.decodeValue slugDecoder f
                |> Result.toMaybe

        lang =
            Decode.decodeValue langDecoder f
                |> Result.toMaybe
                |> Maybe.withDefault En
    in
    ( Model
        RemoteData.Loading
        style
        activeCategory
        Maybe.Nothing
        Maybe.Nothing
        Maybe.Nothing
        Played
        env
        Maybe.Nothing
        jwtToken
        RemoteData.NotAsked
        []
        RemoteData.Loading
        RemoteData.NotAsked
        projectSlug
        0
        RemoteData.Loading
        Maybe.Nothing
        RemoteData.NotAsked
        ""
        lang
    , Cmd.batch
        [ fetchProjects env jwtToken
        , fetchCategories env jwtToken
        ]
    )


addView : Int -> Env -> String -> Cmd Msg
addView projectId env jwtToken =
    Http.request
        { url =
            case env of
                Production ->
                    "https://ctprods.cyprientaque.com/projects/" ++ String.fromInt projectId ++ "/addView"

                _ ->
                    "http://localhost:8088/projects/" ++ String.fromInt projectId ++ "/addView"
        , expect = Http.expectString AddViewResponse
        , method = "PUT"
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


fetchProjects : Env -> String -> Cmd Msg
fetchProjects env jwtToken =
    Http.request
        { url =
            case env of
                Production ->
                    "https://ctprods.cyprientaque.com/projects/all_but_not_blog"

                _ ->
                    "http://localhost:8088/projects/all_but_not_blog"
        , expect = Http.expectJson (RemoteData.fromResult >> GetProjectsResponse) decodeProjects
        , method = "GET"
        , timeout = Nothing
        , tracker = Nothing
        , body = Http.emptyBody
        , headers = [ Http.header "Authorization" ("Bearer " ++ jwtToken) ]
        }


searchProjects : Env -> String -> String -> Int -> Cmd Msg
searchProjects env jwtToken keyword categoryId =
    Http.request
        { url =
            case env of
                Production ->
                    "https://ctprods.cyprientaque.com/projects/search?s=" ++ keyword ++ "&category_id=" ++ String.fromInt categoryId

                _ ->
                    "http://localhost:8088/projects/search?s=" ++ keyword ++ "&category_id=" ++ String.fromInt categoryId
        , expect = Http.expectJson (RemoteData.fromResult >> SearchProjectsResponse) decodeIds
        , method = "GET"
        , timeout = Nothing
        , tracker = Nothing
        , body = Http.emptyBody
        , headers = [ Http.header "Authorization" ("Bearer " ++ jwtToken) ]
        }


fetchCategories : Env -> String -> Cmd Msg
fetchCategories env jwtToken =
    Http.request
        { url =
            case env of
                Production ->
                    "https://ctprods.cyprientaque.com/categories"

                _ ->
                    "http://localhost:8088/categories"
        , expect = Http.expectJson (RemoteData.fromResult >> GetCategoriesResponse) decodeCategories
        , method = "GET"
        , timeout = Nothing
        , tracker = Nothing
        , body = Http.emptyBody
        , headers = [ Http.header "Authorization" ("Bearer " ++ jwtToken) ]
        }


fetchBitbucketProject : String -> String -> Cmd Msg
fetchBitbucketProject accessToken projectKey =
    Http.request
        { url = "https://api.bitbucket.org/2.0/workspaces/Syprex/projects/" ++ projectKey ++ "?access_token=" ++ accessToken
        , expect = Http.expectJson (RemoteData.fromResult >> GetBitbucketProjectResponse) decodeBitbucketProject
        , method = "GET"
        , timeout = Nothing
        , tracker = Nothing
        , body = Http.emptyBody
        , headers = []
        }


fetchRepositories : String -> Int -> Cmd Msg
fetchRepositories accessToken page =
    Http.request
        { url = "https://api.bitbucket.org/2.0/repositories/Syprex?page=" ++ String.fromInt page ++ "&access_token=" ++ accessToken
        , expect = Http.expectJson (RemoteData.fromResult >> GetRepositoriesResponse) decodeBitbucketRepositoryResponse
        , method = "GET"
        , timeout = Nothing
        , tracker = Nothing
        , body = Http.emptyBody
        , headers = []
        }


getAccessToken : Env -> String -> Cmd Msg
getAccessToken env jwtToken =
    Http.request
        { url =
            case env of
                Production ->
                    "https://ctprods.cyprientaque.com/bitbucket/accessToken"

                _ ->
                    "http://localhost:8088/bitbucket/accessToken"
        , expect = Http.expectJson (RemoteData.fromResult >> GetAccessTokenResponse) decodeAccessToken
        , method = "GET"
        , timeout = Nothing
        , tracker = Nothing
        , body = Http.emptyBody
        , headers = [ Http.header "Authorization" ("Bearer " ++ jwtToken) ]
        }


getPrimaryImage : Project -> Maybe Image
getPrimaryImage project =
    List.head (List.filter (\image -> image.primary) project.images)


sortImages : List Image -> List Image
sortImages images =
    images
        |> List.sortBy .views_count
        |> List.sortBy
            (\i ->
                if i.primary == True then
                    1

                else
                    0
            )
        |> List.reverse


getMouseWheelNextData : Maybe Project -> Project -> Maybe Image -> WebData AccessToken -> OnMouseWheelNextData
getMouseWheelNextData next current image accessToken =
    case next of
        Just p ->
            let
                cmd =
                    if p.id == current.id then
                        Cmd.none

                    else
                        case p.bitbucket_project_key of
                            Just key ->
                                case accessToken of
                                    RemoteData.Success token ->
                                        fetchBitbucketProject token.access_token key

                                    _ ->
                                        Cmd.none

                            Nothing ->
                                Cmd.none
            in
            OnMouseWheelNextData p (getPrimaryImage p) cmd

        Nothing ->
            OnMouseWheelNextData current image Cmd.none


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        SearchProjectsResponse response ->
            ( { model | searchProjectsIds = response }, Cmd.none )

        HandleSearch keyword ->
            let
                category_id =
                    case model.realCategory of
                        Just cat ->
                            cat.id

                        Nothing ->
                            0

                cmd =
                    searchProjects model.env model.jwtToken keyword category_id
            in
            case keyword of
                "" ->
                    ( { model | searchProjectsIds = RemoteData.NotAsked, searchKeyword = "" }, Cmd.none )

                _ ->
                    ( { model | searchKeyword = keyword }, cmd )

        ClearSearchInput ->
            ( { model | searchProjectsIds = RemoteData.NotAsked, searchKeyword = "" }, Cmd.none )

        SetRealCategory category ->
            ( { model | realCategory = Maybe.Just category }, Cmd.none )

        GetCategoriesResponse categories ->
            ( { model | categories = categories }, Cmd.none )

        GotBranch branch ->
            case branch of
                "software" ->
                    ( { model | activeCategory = Web }, Cmd.none )

                _ ->
                    ( { model | activeCategory = Other }, Cmd.none )

        OnKeyDown key ->
            case model.projects of
                RemoteData.Success projects ->
                    case model.currentProject of
                        Nothing ->
                            ( model, Cmd.none )

                        Just current ->
                            let
                                keyParsed =
                                    rawValue key

                                filteredProjects =
                                    filterProjects model projects
                            in
                            let
                                currentProjectIndex =
                                    List.Extra.findIndex (getProjectIndex current) filteredProjects

                                nextData =
                                    if model.scrollTimer >= 1 then
                                        case currentProjectIndex of
                                            Just index ->
                                                case keyParsed of
                                                    "ArrowRight" ->
                                                        let
                                                            next =
                                                                List.Extra.getAt (index + 1) filteredProjects
                                                        in
                                                        getMouseWheelNextData next current model.currentImage model.accessToken

                                                    "ArrowLeft" ->
                                                        let
                                                            next =
                                                                List.Extra.getAt (index - 1) filteredProjects
                                                        in
                                                        getMouseWheelNextData next current model.currentImage model.accessToken

                                                    _ ->
                                                        getMouseWheelNextData (Maybe.Just current) current model.currentImage model.accessToken

                                            Nothing ->
                                                getMouseWheelNextData (Maybe.Just current) current model.currentImage model.accessToken

                                    else
                                        getMouseWheelNextData (Maybe.Just current) current model.currentImage model.accessToken
                            in
                            ( { model
                                | currentProject = Maybe.Just nextData.project
                                , scrollTimer =
                                    if current.id == nextData.project.id then
                                        model.scrollTimer

                                    else if model.scrollTimer >= 1 then
                                        0

                                    else
                                        model.scrollTimer
                                , currentImage = nextData.image
                                , bitbucketProject = RemoteData.NotAsked
                              }
                            , Cmd.batch
                                [ nextData.fetchBitbucketProjectCmd
                                , setSlug (Maybe.Just nextData.project.slug)
                                ]
                            )

                _ ->
                    ( model, Cmd.none )

        PlayCarousel ->
            ( { model | carousel = Played }, Cmd.none )

        PauseCarousel ->
            ( { model | carousel = Paused }, Cmd.none )

        CycleImages project currentImage ->
            let
                imgs =
                    sortImages project.images

                currentImageIndex =
                    List.Extra.elemIndex currentImage imgs
                        |> Maybe.withDefault 0

                length =
                    List.length imgs

                nextIndex =
                    if currentImageIndex == length - 1 then
                        0

                    else
                        currentImageIndex
                            + 1

                nextImage =
                    List.Extra.getAt
                        nextIndex
                        imgs
            in
            ( { model | currentImage = nextImage }, Cmd.none )

        ImgLoaded dimensions ->
            ( { model | imageLoaded = Just dimensions }, Cmd.none )

        GetProjectsResponse projects ->
            let
                newData =
                    case projects of
                        RemoteData.Success realProjects ->
                            case model.urlSlug of
                                Just slug ->
                                    let
                                        found =
                                            realProjects
                                                |> List.filter (\p -> p.slug == slug)
                                                |> List.head

                                        image =
                                            case found of
                                                Just p ->
                                                    List.head (List.filter (\img -> img.primary) p.images)

                                                Nothing ->
                                                    Maybe.Nothing

                                        addViewCmd =
                                            case found of
                                                Just p ->
                                                    addView p.id model.env model.jwtToken

                                                Nothing ->
                                                    Cmd.none

                                        category =
                                            case found of
                                                Just p ->
                                                    if p.category_id == 1 then
                                                        Web

                                                    else
                                                        Other

                                                Nothing ->
                                                    model.activeCategory
                                    in
                                    case found of
                                        Just p ->
                                            GetProjectsResponseNewData (Maybe.Just p) image addViewCmd category

                                        Nothing ->
                                            GetProjectsResponseNewData Maybe.Nothing Maybe.Nothing Cmd.none model.activeCategory

                                Nothing ->
                                    GetProjectsResponseNewData Maybe.Nothing Maybe.Nothing Cmd.none model.activeCategory

                        _ ->
                            GetProjectsResponseNewData Maybe.Nothing Maybe.Nothing Cmd.none model.activeCategory

                setBranchPortCmd =
                    case newData.category of
                        Web ->
                            setBranch "software"

                        _ ->
                            setBranch "other"
            in
            ( { model
                | projects = projects
                , currentProject = newData.project
                , currentImage = newData.image
                , activeCategory = newData.category
              }
            , Cmd.batch [ newData.addViewCmd, getAccessToken model.env model.jwtToken, setBranchPortCmd ]
            )

        GetAccessTokenResponse resp ->
            let
                fetchReposAndBitbucketProject =
                    case resp of
                        RemoteData.Success tokenData ->
                            Cmd.batch
                                [ fetchRepositories tokenData.access_token 1
                                , case model.currentProject of
                                    Just p ->
                                        case p.bitbucket_project_key of
                                            Just key ->
                                                fetchBitbucketProject tokenData.access_token key

                                            Nothing ->
                                                Cmd.none

                                    Nothing ->
                                        Cmd.none
                                ]

                        _ ->
                            Cmd.none
            in
            ( { model | accessToken = resp }, fetchReposAndBitbucketProject )

        GetRepositoriesResponse response ->
            let
                cmd =
                    case response of
                        RemoteData.Success value ->
                            case value.next of
                                Nothing ->
                                    Cmd.none

                                _ ->
                                    case model.accessToken of
                                        RemoteData.Success token ->
                                            fetchRepositories token.access_token (value.page + 1)

                                        _ ->
                                            Cmd.none

                        _ ->
                            Cmd.none

                currentList =
                    model.repositoriesFullList

                newList =
                    case response of
                        RemoteData.Success newResponse ->
                            newResponse.values

                        _ ->
                            []
            in
            ( { model | lastRepositoriesFetch = response, repositoriesFullList = List.concat [ currentList, newList ] }, cmd )

        SetCategory category ->
            case category of
                Web ->
                    let
                        cmd =
                            setBranch "software"
                    in
                    ( { model | activeCategory = Web }, cmd )

                Other ->
                    let
                        cmd =
                            setBranch "other"
                    in
                    ( { model | activeCategory = Other }, cmd )

        AddViewResponse res ->
            ( model, Cmd.none )

        AddImageViewResponse res ->
            ( model, Cmd.none )

        SetCurrentProject project ->
            case project of
                Just p ->
                    let
                        accessToken =
                            case model.accessToken of
                                RemoteData.Success token ->
                                    token.access_token

                                _ ->
                                    ""

                        fetchBitbucketProjectCmd =
                            if String.length accessToken > 0 then
                                case p.bitbucket_project_key of
                                    Just key ->
                                        fetchBitbucketProject accessToken key

                                    Nothing ->
                                        Cmd.none

                            else
                                Cmd.none
                    in
                    ( { model
                        | currentProject = Maybe.Just p
                        , bitbucketProject = RemoteData.NotAsked
                      }
                    , Cmd.batch
                        [ addView p.id model.env model.jwtToken
                        , Delay.after 300 Delay.Millisecond (SetPrimaryImage p)
                        , fetchBitbucketProjectCmd
                        , setSlug (Maybe.Just p.slug)
                        ]
                    )

                Nothing ->
                    ( { model
                        | currentProject = Maybe.Nothing
                        , currentImage = Maybe.Nothing
                        , imageLoaded = Maybe.Nothing
                        , bitbucketProject = RemoteData.NotAsked
                      }
                    , setSlug Maybe.Nothing
                    )

        GetBitbucketProjectResponse p ->
            ( { model | bitbucketProject = p }, Cmd.none )

        SetPrimaryImage project ->
            let
                maybePrimaryImage : Maybe Image
                maybePrimaryImage =
                    getPrimaryImage project
            in
            ( { model | currentImage = maybePrimaryImage }
            , Cmd.none
            )

        SetCurrentImage image ->
            case image of
                Just i ->
                    ( { model | currentImage = Maybe.Just i, carousel = Paused }, addImageView i.id model.env model.jwtToken )

                Nothing ->
                    ( { model | currentImage = Maybe.Nothing }, Cmd.none )

        OnTime posix ->
            let
                secSinceEpoch =
                    posixToMillis posix // 1000
            in
            ( { model | secSinceEpoch = Maybe.Just secSinceEpoch }, Cmd.none )

        GotJwt jwtToken ->
            ( { model | jwtToken = jwtToken }, Cmd.none )

        GotSlug maybeSlug ->
            case maybeSlug of
                Just s ->
                    case model.projects of
                        RemoteData.Success projects ->
                            let
                                found =
                                    projects
                                        |> List.filter (\p -> p.slug == s)
                                        |> List.head
                            in
                            case found of
                                Just project ->
                                    let
                                        accessToken =
                                            case model.accessToken of
                                                RemoteData.Success token ->
                                                    token.access_token

                                                _ ->
                                                    ""

                                        fetchBitbucketProjectCmd =
                                            if String.length accessToken > 0 then
                                                case project.bitbucket_project_key of
                                                    Just key ->
                                                        fetchBitbucketProject accessToken key

                                                    Nothing ->
                                                        Cmd.none

                                            else
                                                Cmd.none
                                    in
                                    ( { model | currentProject = Maybe.Just project }
                                    , Cmd.batch
                                        [ addView project.id model.env model.jwtToken
                                        , Delay.after 300 Delay.Millisecond (SetPrimaryImage project)
                                        , fetchBitbucketProjectCmd
                                        ]
                                    )

                                Nothing ->
                                    ( model, Cmd.none )

                        _ ->
                            ( model, Cmd.none )

                Nothing ->
                    ( { model | currentProject = Maybe.Nothing, currentImage = Maybe.Nothing }, Cmd.none )

        OnMouseWheel current list direction ->
            let
                currentProjectIndex =
                    List.Extra.findIndex (getProjectIndex current) list

                nextData =
                    if model.scrollTimer >= 3 then
                        case currentProjectIndex of
                            Just index ->
                                case direction of
                                    Up ->
                                        let
                                            next =
                                                List.Extra.getAt (index + 1) list
                                        in
                                        getMouseWheelNextData next current model.currentImage model.accessToken

                                    Down ->
                                        let
                                            next =
                                                List.Extra.getAt (index - 1) list
                                        in
                                        getMouseWheelNextData next current model.currentImage model.accessToken

                            Nothing ->
                                getMouseWheelNextData (Maybe.Just current) current model.currentImage model.accessToken

                    else
                        getMouseWheelNextData (Maybe.Just current) current model.currentImage model.accessToken
            in
            ( { model
                | currentProject = Maybe.Just nextData.project
                , bitbucketProject = RemoteData.NotAsked
                , scrollTimer =
                    if current.id == nextData.project.id then
                        model.scrollTimer

                    else if model.scrollTimer >= 3 then
                        0

                    else
                        model.scrollTimer
                , currentImage = nextData.image
              }
            , Cmd.batch
                [ nextData.fetchBitbucketProjectCmd
                , setSlug (Maybe.Just nextData.project.slug)
                ]
            )

        SetScrollTimer time ->
            ( { model | scrollTimer = model.scrollTimer + 1 }, Cmd.none )


filterActiveCategory : Model -> Project -> Bool
filterActiveCategory model project =
    case model.realCategory of
        Just cat ->
            case cat.id of
                0 ->
                    True

                _ ->
                    project.category_id == cat.id

        Nothing ->
            True


onProjectClick : msg -> Html.Attribute msg
onProjectClick msg =
    stopPropagationOn "click" (Decode.map alwaysStopPropagation (Decode.succeed msg))


alwaysStopPropagation : msg -> ( msg, Bool )
alwaysStopPropagation msg =
    ( msg, True )


onMouseWheel : Project -> List Project -> Wheel.Event -> Msg
onMouseWheel current list wheelEvent =
    if wheelEvent.deltaY > 0 then
        OnMouseWheel current list Up

    else
        OnMouseWheel current list Down


mapProject : Model -> String -> Int -> List Project -> Int -> Project -> Html Msg
mapProject model transform currentIndex projects i project =
    let
        maybePrimaryImage : Maybe Image
        maybePrimaryImage =
            List.head (List.filter (\image -> image.primary) project.images)

        maybeAuthorImage : Maybe ProfileImage
        maybeAuthorImage =
            List.head project.user.profile_images
    in
    a
        [ class model.style.portfolio "listItem"
        , style "transform"
            (transform
                ++ (if currentIndex == i then
                        " scale(1.1)"

                    else if transform /= "" then
                        " scale(.9)"

                    else
                        ""
                   )
            )
        , onProjectClick (SetCurrentProject (Maybe.Just project))
        , case model.currentProject of
            Just p ->
                Wheel.onWheel (onMouseWheel p projects)

            Nothing ->
                attribute "wheel" "none"
        ]
        [ case maybePrimaryImage of
            Just gotImage ->
                img [ attribute "src" gotImage.w350_object_url ] []

            Nothing ->
                img [] []
        , div [ class model.style.portfolio "content" ]
            [ case maybeAuthorImage of
                Just image ->
                    img
                        [ attribute "src" image.w40_object_url
                        , class model.style.portfolio "avatar"
                        ]
                        []

                Nothing ->
                    div [] []
            , h3 [] [ text project.title ]
            , div [ class model.style.portfolio "date-pro-tag" ]
                [ div [ class model.style.portfolio "date-category-color-wrapper" ]
                    [ span [ class model.style.portfolio "date" ] [ text (formatI18n french "dddd, dd MMMM yyyy" utc project.created_at) ]
                    , div
                        [ class model.style.portfolio "dot-color"
                        , style "background-color" project.category.color_hex
                        ]
                        [ div
                            [ class model.style.portfolio "category-tooltip" ]
                            [ span [] [ text project.category.name ]
                            ]
                        ]
                    ]
                , case project.is_pro of
                    True ->
                        span [ class model.style.portfolio "pro-tag" ] [ text "pro" ]

                    False ->
                        div [] []
                ]
            ]
        ]


imageThumbnail : Model -> Image -> Html Msg
imageThumbnail model image =
    button [ class model.style.portfolio "thumb", image |> Maybe.Just |> SetCurrentImage |> onClick ]
        [ img [ attribute "src" image.w350_object_url ] []
        ]


loader : Model -> Html msg
loader model =
    div [ class model.style.portfolio "loader-wrapper" ]
        [ div [ class model.style.portfolio "lds-ring" ]
            [ div [] []
            , div [] []
            , div [] []
            ]
        ]


getProjectIndex : Project -> Project -> Bool
getProjectIndex currentProject current =
    currentProject.id == current.id


filterProjects : Model -> List Project -> List Project
filterProjects model projects =
    case model.searchProjectsIds of
        RemoteData.Success ids ->
            projects
                |> List.filter (filterActiveCategory model)
                |> List.filter (\p -> List.any (\id -> p.id == id) ids)
                |> List.sortBy (\project -> project.views_count)
                |> List.reverse

        _ ->
            projects
                |> List.filter (filterActiveCategory model)
                |> List.sortBy (\project -> project.views_count)
                |> List.reverse


projectDetailSider : Model -> Html Msg
projectDetailSider model =
    div [ class model.style.portfolio "projectDetailsSider--componentWrapper" ]
        [ div
            [ class model.style.portfolio "projectDetailsSiderMask"
            , onClick (SetCurrentProject Maybe.Nothing)
            , case model.currentProject of
                Just p ->
                    style "width" "calc(100vw - 600px)"

                Nothing ->
                    style "width" "0px"
            , case model.currentProject of
                Just p ->
                    style "transition" "width .3s ease"

                Nothing ->
                    style "transition" "inherit"
            , case model.currentProject of
                Just p ->
                    style "min-width" "20vw"

                Nothing ->
                    style "min-width" "0px"
            ]
            [ case model.currentProject of
                Just current ->
                    div [ class model.style.portfolio "projectDetailsSiderMask--inner-wrapper" ]
                        [ case model.projects of
                            RemoteData.Success projects ->
                                let
                                    filteredProjects =
                                        filterProjects model projects

                                    currentProjectIndex =
                                        let
                                            mIndex =
                                                List.Extra.findIndex (getProjectIndex current) filteredProjects
                                        in
                                        case mIndex of
                                            Just index ->
                                                index

                                            Nothing ->
                                                0
                                in
                                div
                                    [ class model.style.portfolio "listWrapper"
                                    ]
                                    (filteredProjects
                                        |> List.indexedMap (mapProject model ("translateX(-" ++ String.fromInt ((currentProjectIndex * 300) + 150) ++ "px) translateX(-1em)") currentProjectIndex filteredProjects)
                                    )

                            _ ->
                                div [] []
                        ]

                Nothing ->
                    div [] []
            ]
        , div
            [ case model.currentProject of
                Just p ->
                    classList model.style.portfolio [ ( "projectDetailsSider", True ), ( "open", True ) ]

                Nothing ->
                    classList model.style.portfolio [ ( "projectDetailsSider", True ), ( "open", False ) ]
            ]
            [ case model.currentProject of
                Just p ->
                    let
                        cmpStyle =
                            model.style.portfolio
                    in
                    div [ class cmpStyle "content" ]
                        [ div
                            [ class cmpStyle "primary-image--wrapper"
                            , case model.imageLoaded of
                                Just dimensions ->
                                    style "height" (String.fromInt dimensions.height ++ "px")

                                Nothing ->
                                    style "height" "0px"
                            ]
                            [ case model.currentImage of
                                Just image ->
                                    img
                                        [ class cmpStyle "primary"
                                        , on "load" (decodeImageLoaded ImgLoaded)
                                        , attribute "src" image.w1500_object_url
                                        ]
                                        []

                                Nothing ->
                                    div [] []
                            , div [ class cmpStyle "footer" ]
                                [ div [ class cmpStyle "controls" ]
                                    [ case model.carousel of
                                        Paused ->
                                            button [ onClick PlayCarousel ]
                                                [ Svg.svg
                                                    [ Svg.Attributes.width "30"
                                                    , Svg.Attributes.height "30"
                                                    , Svg.Attributes.fill "#524e59"
                                                    , Svg.Attributes.viewBox "0 0 24 24"
                                                    ]
                                                    [ Svg.path [ Svg.Attributes.d "M8 5v14l11-7z" ] []
                                                    , Svg.path [ Svg.Attributes.d "M0 0h24v24H0z", Svg.Attributes.fill "none" ] []
                                                    ]
                                                ]

                                        _ ->
                                            button [ onClick PauseCarousel ]
                                                [ Svg.svg
                                                    [ Svg.Attributes.width "30"
                                                    , Svg.Attributes.height "30"
                                                    , Svg.Attributes.fill "#524e59"
                                                    , Svg.Attributes.viewBox "0 0 24 24"
                                                    ]
                                                    [ Svg.path [ Svg.Attributes.d "M6 19h4V5H6v14zm8-14v14h4V5h-4z" ] []
                                                    , Svg.path [ Svg.Attributes.d "M0 0h24v24H0z", Svg.Attributes.fill "none" ] []
                                                    ]
                                                ]
                                    ]
                                ]
                            ]
                        , div [ class cmpStyle "thumb-wrapper" ]
                            (sortImages p.images
                                |> List.map (imageThumbnail model)
                            )
                        , div [ class model.style.portfolio "innerContent" ]
                            [ div [ class model.style.portfolio "titleWrapper" ]
                                [ h1 []
                                    [ div
                                        [ class model.style.portfolio "eyeViewsWrapper" ]
                                        [ Svg.svg
                                            [ Svg.Attributes.width "20"
                                            , Svg.Attributes.height "20"
                                            , Svg.Attributes.fill "#524e59"
                                            , Svg.Attributes.viewBox "0 0 1024 1024"
                                            ]
                                            [ Svg.path
                                                [ Svg.Attributes.d "M512 199.0656C204.8 199.0656 56.9344 497.4592 56.9344 512c0 14.6432 143.9744 312.9344 455.0656 312.9344 311.0912 0 455.0656-298.2912 455.0656-312.9344 0-14.5408-147.8656-312.9344-455.0656-312.9344z m0 483.6352c-94.208 0-170.7008-76.4928-170.7008-170.7008 0-94.208 76.3904-170.7008 170.7008-170.7008 94.208 0 170.7008 76.3904 170.7008 170.7008 0 94.208-76.4928 170.7008-170.7008 170.7008z m0-256c-47.104 0-85.2992 38.1952-85.2992 85.2992s38.1952 85.2992 85.2992 85.2992 85.2992-38.1952 85.2992-85.2992-38.1952-85.2992-85.2992-85.2992z"
                                                , Svg.Attributes.fill "#ffffffcc"
                                                ]
                                                []
                                            ]
                                        , span [] [ text <| String.fromInt <| p.views_count ]
                                        ]
                                    , text p.title
                                    ]
                                , div [ class cmpStyle "btnsWrapper" ]
                                    [ small
                                        [ class model.style.portfolio "tag" ]
                                        [ text p.category.name ]
                                    , case p.is_pro of
                                        True ->
                                            small
                                                [ class model.style.portfolio "tag" ]
                                                [ text "Projet professionnel" ]

                                        False ->
                                            div [] []
                                    ]
                                ]
                            , case model.bitbucketProject of
                                RemoteData.Success bitbucketProject ->
                                    case bitbucketProject.is_private of
                                        True ->
                                            div [] []

                                        False ->
                                            div [ class model.style.portfolio "bitbucket_project_wrapper" ]
                                                [ div [ class model.style.portfolio "innerWrapper" ]
                                                    [ img
                                                        [ class model.style.portfolio "image"
                                                        , attribute "src" bitbucketProject.links.avatar.href
                                                        ]
                                                        []
                                                    , div
                                                        [ class model.style.portfolio "contentOuterWrapper" ]
                                                        [ div [ class model.style.portfolio "header" ]
                                                            [ h4 [ class model.style.portfolio "title" ]
                                                                [ text "Code source du projet"
                                                                ]
                                                            ]
                                                        , div [ class model.style.portfolio "content" ]
                                                            [ Html.p [] [ text bitbucketProject.description ]
                                                            , span [ class model.style.portfolio "created_on" ]
                                                                [ text
                                                                    ("Créé le "
                                                                        ++ formatI18n french "dd MMMM yyyy" utc bitbucketProject.created_on
                                                                    )
                                                                ]
                                                            ]
                                                        ]
                                                    ]
                                                , div [ class model.style.portfolio "footer" ]
                                                    [ a
                                                        [ class model.style.portfolio "linkToProject"
                                                        , attribute "href" bitbucketProject.links.html.href
                                                        , attribute "target" "_blank"
                                                        ]
                                                        [ text ("Voir " ++ bitbucketProject.name ++ " sur bitbucket")
                                                        , span [ class model.style.portfolio "rightArrow" ]
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

                                RemoteData.Loading ->
                                    loader model

                                RemoteData.NotAsked ->
                                    case p.bitbucket_project_key of
                                        Just key ->
                                            loader model

                                        Nothing ->
                                            div [] []

                                _ ->
                                    div [] []
                            , div [ class model.style.portfolio "markdown" ]
                                [ Markdown.toHtml [] p.content
                                ]
                            ]
                        ]

                Nothing ->
                    div [] []
            ]
        ]


projectPlaceholder : Model -> Int -> Html Msg
projectPlaceholder model idx =
    div [ class model.style.portfolio "projectPlaceholder" ] []


mapRepositories : Model -> List Repository -> List (Html Msg)
mapRepositories model repos =
    repos
        |> List.filter (\repo -> repo.is_private == False)
        |> List.map
            (\repo ->
                div [ class model.style.portfolio "repositoryItem" ]
                    [ div [ class model.style.portfolio "header" ]
                        [ h1 []
                            [ svg
                                [ Svg.Attributes.width "40"
                                , Svg.Attributes.height "40"
                                , Svg.Attributes.viewBox "0 0 1024 1024"
                                ]
                                [ Svg.path
                                    [ Svg.Attributes.d "M305.6 225.6c-17.6-17.6-43.2-17.6-59.2 0L19.2 460.8c-25.6 30.4-25.6 72 0 97.6l225.6 235.2c8 8 20.8 12.8 30.4 12.8s20.8-4.8 30.4-12.8c17.6-17.6 17.6-43.2 0-59.2L88 512l217.6-225.6c17.6-17.6 17.6-43.2 0-60.8zM1001.6 460.8L774.4 225.6c-17.6-17.6-43.2-17.6-59.2 0s-17.6 43.2 0 59.2L932.8 512 715.2 737.6c-17.6 17.6-17.6 43.2 0 59.2 8 8 17.6 12.8 30.4 12.8 12.8 0 20.8-4.8 30.4-12.8l225.6-235.2c28.8-28.8 28.8-70.4 0-100.8zM612.8 230.4c-20.8-8-46.4 4.8-56 25.6L382.4 742.4c-8 20.8 4.8 46.4 25.6 56 4.8 0 8 4.8 12.8 4.8 17.6 0 33.6-12.8 38.4-30.4l179.2-491.2c8-20.8-4.8-46.4-25.6-51.2z"
                                    , Svg.Attributes.fill "#f0bf00"
                                    ]
                                    []
                                ]
                            ]
                        , h3 [ class model.style.portfolio "title" ] [ text repo.name ]
                        ]
                    , div [ class model.style.portfolio "content" ]
                        [ div []
                            [ case String.length repo.description of
                                0 ->
                                    div [] []

                                _ ->
                                    p [] [ text repo.description ]
                            , small [ class model.style.portfolio "created_on" ]
                                [ text ("Créé le " ++ formatI18n french "dd MMMM yyyy" utc repo.created_on)
                                ]
                            , small [ class model.style.portfolio "updated_on" ]
                                [ text ("Mis à jour le " ++ formatI18n french "dd MMMM yyyy" utc repo.updated_on)
                                ]
                            ]
                        ]
                    , div [ class model.style.portfolio "footer" ]
                        [ case String.length repo.language of
                            0 ->
                                div [] []

                            _ ->
                                span [ class model.style.portfolio "tag" ] [ text repo.language ]
                        , div [ style "flex" "3" ] []
                        , a [ attribute "target" "_blank", attribute "href" repo.links.html.href ] [ text "Voir le dépot" ]
                        , div [ class model.style.portfolio "linkArrow" ]
                            [ svg
                                [ Svg.Attributes.width "20"
                                , Svg.Attributes.height "20"
                                , Svg.Attributes.viewBox "0 0 24 24"
                                ]
                                [ Svg.path [ Svg.Attributes.d "M0 0h24v24H0z", Svg.Attributes.fill "none" ] []
                                , Svg.path
                                    [ Svg.Attributes.d "M20 11H7.83l5.59-5.59L12 4l-8 8 8 8 1.41-1.41L7.83 13H20v-2z"
                                    , Svg.Attributes.fill "#f0bf00"
                                    ]
                                    []
                                ]
                            ]
                        ]
                    ]
            )


mapCategories : Model -> Category -> Html Msg
mapCategories model category =
    button
        [ classList model.style.portfolio
            [ ( "categoryButton", True )
            , ( "inactive"
              , case model.realCategory of
                    Just cat ->
                        cat.id /= category.id

                    Nothing ->
                        category.id /= 0
              )
            ]
        , style "box-shadow" ("0px 0px 1px 2px " ++ category.color_hex)
        , onClick (SetRealCategory category)
        ]
        [ div [ style "background-color" category.color_hex ] []
        , span [] [ text category.name ]
        , case category.project_count of
            Just count ->
                span [] [ text ("(" ++ String.fromInt count ++ ")") ]

            Nothing ->
                div [] []
        ]


activeCategoryToParentSlug : FilterCategory -> String
activeCategoryToParentSlug category =
    case category of
        Web ->
            "development"

        Other ->
            "wood&composites"


view : Model -> Html Msg
view model =
    case model.projects of
        RemoteData.Success projects ->
            div [ class model.style.portfolio "portfolio" ]
                [ projectDetailSider model
                , case model.categories of
                    RemoteData.Success categories ->
                        let
                            item : Category
                            item =
                                { id = 0
                                , name = "Tout"
                                , slug = "all"
                                , color_hex = "#fff"
                                , picture_url = ""
                                , project_count = Maybe.Nothing
                                , parent_id = 0
                                , parent_category = Maybe.Nothing
                                }

                            filtered : List Category
                            filtered =
                                categories
                                    |> List.filter (\cat -> cat.id /= 5)
                                    |> List.filter (\cat -> cat.parent_id /= 0)

                            groups : Dict String (List Category)
                            groups =
                                DictExtra.groupBy
                                    (\cat ->
                                        case cat.parent_category of
                                            Just parent ->
                                                parent.slug

                                            Nothing ->
                                                "nothing"
                                    )
                                    filtered

                            toMap =
                                Dict.get (activeCategoryToParentSlug model.activeCategory) groups

                            withAllBtn =
                                case toMap of
                                    Just current ->
                                        item :: current

                                    Nothing ->
                                        List.singleton item
                        in
                        div [ class model.style.portfolio "buttons-wrapper" ]
                            (List.map (mapCategories model) withAllBtn)

                    RemoteData.Loading ->
                        loader model

                    _ ->
                        div [] []
                , div [ class model.style.portfolio "searchInputWrapper" ]
                    [ input
                        [ attribute "type" "text"
                        , attribute "placeholder"
                            (case model.lang of
                                Fr ->
                                    "Rechercher"

                                _ ->
                                    "Search"
                            )
                        , onInput HandleSearch
                        , value model.searchKeyword
                        ]
                        []
                    , button [ onClick ClearSearchInput ]
                        [ svg
                            [ Svg.Attributes.width "20"
                            , Svg.Attributes.height "20"
                            , Svg.Attributes.viewBox "0 0 24 24"
                            , Svg.Attributes.fill "black"
                            ]
                            [ Svg.path [ Svg.Attributes.d "M0 0h24v24H0z", Svg.Attributes.fill "none" ] []
                            , Svg.path
                                [ Svg.Attributes.d "M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm5 13.59L15.59 17 12 13.41 8.41 17 7 15.59 10.59 12 7 8.41 8.41 7 12 10.59 15.59 7 17 8.41 13.41 12 17 15.59z"
                                , Svg.Attributes.fill "#505458"
                                ]
                                []
                            ]
                        ]
                    ]
                , div [ class model.style.portfolio "listWrapper" ]
                    (let
                        filteredProjects =
                            filterProjects model projects
                     in
                     filteredProjects
                        |> List.indexedMap (mapProject model "" -1 filteredProjects)
                    )
                , div []
                    [ div [ class model.style.portfolio "listWrapper" ] (mapRepositories model model.repositoriesFullList) ]
                , case model.lastRepositoriesFetch of
                    RemoteData.Success response ->
                        div [] []

                    RemoteData.Failure err ->
                        div [] [ text (errorToString err) ]

                    _ ->
                        loader model
                ]

        RemoteData.Failure err ->
            div [] [ text (errorToString err) ]

        _ ->
            div [ class model.style.portfolio "portfolio" ]
                [ div [ class model.style.portfolio "placeholderWrapper" ]
                    [ div [ class model.style.portfolio "categoriesPlaceholder" ]
                        [ div [] []
                        , div [] []
                        , div [] []
                        , div [] []
                        , div [] []
                        ]
                    , div [ class model.style.portfolio "searchPlaceholder" ] []
                    , div [ class model.style.portfolio "projectsPlaceholderWrapper" ]
                        (List.range 0 9
                            |> List.map (projectPlaceholder model)
                        )
                    ]
                ]


port getJwt : (String -> msg) -> Sub msg


port setSlug : Maybe String -> Cmd msg


port getSlug : (Maybe String -> msg) -> Sub msg


port setBranch : String -> Cmd msg


port getBranch : (String -> msg) -> Sub msg


subscriptions : Model -> Sub Msg
subscriptions model =
    let
        cycleImages =
            case model.currentProject of
                Just p ->
                    case model.currentImage of
                        Just i ->
                            case model.carousel of
                                Played ->
                                    Time.every 3000 (\_ -> CycleImages p i)

                                Paused ->
                                    Sub.none

                        Nothing ->
                            Sub.none

                Nothing ->
                    Sub.none
    in
    Sub.batch
        [ cycleImages
        , Time.every 1000 OnTime
        , Time.every 300 SetScrollTimer
        , getJwt GotJwt
        , getSlug GotSlug
        , Keyboard.downs OnKeyDown
        , getBranch GotBranch
        ]
