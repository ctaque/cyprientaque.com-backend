port module Elm.Components.Skills exposing (..)

import Browser
import Browser.Dom exposing (Element, getElement)
import Browser.Events as Events
import CssClass exposing (CssClass, class, classList)
import DateFormat exposing (format, formatI18n, french)
import Elm.Decoders.Index exposing (filterCategoryDecoder, langDecoder)
import Html exposing (Html, a, button, div, h1, h3, img, nav, small, span, text)
import Html.Attributes exposing (attribute, style)
import Html.Events exposing (onClick)
import Html.Events.Extra.Mouse exposing (Event, onDown, onEnter, onLeave, onOut, onOver)
import Http
import Json.Decode as Decode exposing (Decoder, Error, Value, decodeString, float, int, nullable)
import Json.Decode.Pipeline exposing (optional, required, resolve)
import List.Extra
import Markdown
import Process
import RemoteData exposing (WebData)
import Svg exposing (svg)
import Svg.Attributes
import Task
import Time exposing (utc)
import Types.Index exposing (AlgoliaResponse, Author, Category, FilterCategory(..), Image, Lang(..), Project)


main : Program Flags Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


type Msg
    = GotData (WebData Wrapper)
    | GetYear Time.Posix
    | ShowDetailDefer ( Float, Float ) ( Float, Float ) Techno
    | ShowDetail ( Float, Float ) ( Float, Float ) Techno
    | ShowNothing
    | ShowNothingDefer Event
    | GetRelativeReferenceInterval Time.Posix
    | GetRelativeReference (Result Browser.Dom.Error Element)
    | DoNothing
    | HideDetails
    | GetViewportOfComponent Time.Posix
    | GetTimelineViewport (Result Browser.Dom.Error Element)
    | OnNewBranch String
    | OnDetailDivEnter Event
    | OnDetailDivLeave Event


type alias Dates =
    { start : Int
    , end : Maybe Int
    }


type alias Wrapper =
    { wood : List Techno
    , software : List Techno
    }


type alias Techno =
    { name : String
    , dates : Dates
    , en : Maybe String
    , fr : Maybe String
    }


type alias DetailShown =
    Maybe
        { techno : Techno
        , position : ( Float, Float )
        , offsetFromParent : ( Float, Float )
        }


type alias Model =
    { style : Style
    , data : WebData Wrapper
    , currentYear : Maybe Int
    , detailShown : DetailShown
    , pageRelativeReference : Maybe Element
    , triggerAnimation : Bool
    , category : FilterCategory
    , lang : Lang
    , hoveringDetailsDiv : Bool
    , hoveringSkillBarDiv : Bool
    }


type alias Flags =
    Value


type alias Style =
    { class : CssClass
    }


type DomNode
    = RootNode { id : String }
    | ChildNode { id : String, parentNode : DomNode }


cssDecoder : Decoder Style
cssDecoder =
    Decode.map Style
        (Decode.field "style" CssClass.decode)


datesDecoder : Decoder Dates
datesDecoder =
    Decode.succeed Dates
        |> required "start" int
        |> optional "end" (nullable int) Maybe.Nothing


technoDecoder : Decoder Techno
technoDecoder =
    Decode.succeed Techno
        |> required "name" Decode.string
        |> required "dates" datesDecoder
        |> optional "en" (nullable Decode.string) Maybe.Nothing
        |> optional "fr" (nullable Decode.string) Maybe.Nothing


dataDecoder : Decoder Wrapper
dataDecoder =
    Decode.succeed Wrapper
        |> required "wood" (Decode.list technoDecoder)
        |> required "software" (Decode.list technoDecoder)


domNodeDecoder : Decode.Decoder DomNode
domNodeDecoder =
    Decode.oneOf [ childNode, rootNode ]


rootNode : Decode.Decoder DomNode
rootNode =
    Decode.map (\x -> RootNode { id = x })
        (Decode.field "id" Decode.string)


childNode : Decode.Decoder DomNode
childNode =
    Decode.map2 (\id parentNode -> ChildNode { id = id, parentNode = parentNode })
        (Decode.field "id" Decode.string)
        (Decode.field "parentNode" (Decode.lazy (\_ -> domNodeDecoder)))


isOutSideDetailsDiv : String -> Decode.Decoder Bool
isOutSideDetailsDiv divId =
    Decode.oneOf
        [ Decode.field "id" Decode.string
            |> Decode.andThen
                (\id ->
                    if divId == id then
                        Decode.succeed False

                    else
                        Decode.fail "continue"
                )
        , Decode.lazy (\_ -> isOutSideDetailsDiv divId |> Decode.field "parentNode")
        , Decode.succeed True
        ]


isOutSideDetailsTarget : String -> Decode.Decoder Msg
isOutSideDetailsTarget divId =
    Decode.field "target" (isOutSideDetailsDiv divId)
        |> Decode.andThen
            (\isOutside ->
                if isOutside then
                    Decode.succeed HideDetails

                else
                    Decode.fail "inside"
            )


getData =
    Http.get
        { url = "https://s3.eu-west-3.amazonaws.com/ctaque.divers/technos-skills.json"
        , expect = Http.expectJson (RemoteData.fromResult >> GotData) dataDecoder
        }


init : Flags -> ( Model, Cmd Msg )
init f =
    let
        style =
            Decode.decodeValue cssDecoder f
                |> Result.toMaybe
                |> Maybe.withDefault
                    { class = CssClass.empty }

        getViewportTask =
            getElement "pageRelativeReference" |> Task.attempt GetRelativeReference

        filterCategory =
            Decode.decodeValue filterCategoryDecoder f
                |> Result.toMaybe
                |> Maybe.withDefault Web

        lang =
            Decode.decodeValue langDecoder f
                |> Result.toMaybe
                |> Maybe.withDefault En
    in
    ( Model style RemoteData.Loading Maybe.Nothing Maybe.Nothing Maybe.Nothing False filterCategory lang False False
    , Cmd.batch [ getData, getViewportTask ]
    )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        OnNewBranch branch ->
            let
                category =
                    case branch of
                        "software" ->
                            Web

                        _ ->
                            Other
            in
            ( { model | category = category }, Cmd.none )

        GetYear posix ->
            let
                year =
                    format "yyyy" Time.utc posix
            in
            ( { model | currentYear = String.toInt year }, Cmd.none )

        GotData data ->
            ( { model | data = data }, Cmd.none )

        ShowNothingDefer event ->
            let
                cmd =
                    Process.sleep 300
                        |> Task.perform (\_ -> ShowNothing)
            in
            ( { model | hoveringSkillBarDiv = False }, cmd )

        ShowNothing ->
            case model.hoveringDetailsDiv of
                True ->
                    ( model, Cmd.none )

                False ->
                    case model.hoveringSkillBarDiv of
                        True ->
                            ( model, Cmd.none )

                        False ->
                            ( { model | detailShown = Maybe.Nothing }, Cmd.none )

        OnDetailDivEnter event ->
            ( { model | hoveringDetailsDiv = True }, Cmd.none )

        OnDetailDivLeave event ->
            ( { model | hoveringDetailsDiv = False, detailShown = Maybe.Nothing }, Cmd.none )

        HideDetails ->
            ( { model | detailShown = Maybe.Nothing }, Cmd.none )

        ShowDetailDefer position offset techno ->
            let
                sleepTask =
                    Process.sleep 0.5
                        |> Task.perform (\_ -> ShowDetail position offset techno)
            in
            ( { model | hoveringSkillBarDiv = True }, sleepTask )

        ShowDetail position offset techno ->
            ( { model | detailShown = Maybe.Just { techno = techno, position = position, offsetFromParent = offset } }, Cmd.none )

        GetRelativeReferenceInterval time ->
            let
                cmd =
                    getElement "skills"
                        |> Task.attempt GetRelativeReference
            in
            ( model, cmd )

        GetViewportOfComponent time ->
            let
                cmd =
                    getElement "timelineWrapper"
                        |> Task.attempt GetTimelineViewport
            in
            ( model, cmd )

        GetRelativeReference value ->
            case value of
                Err (Browser.Dom.NotFound id) ->
                    ( model, Cmd.none )

                Ok element ->
                    ( { model | pageRelativeReference = Maybe.Just element }, Cmd.none )

        GetTimelineViewport value ->
            case value of
                Err (Browser.Dom.NotFound id) ->
                    ( model, Cmd.none )

                Ok element ->
                    if element.element.y <= (element.viewport.y + element.viewport.height - 150) then
                        ( { model | triggerAnimation = True }, Cmd.none )

                    else
                        ( { model | triggerAnimation = False }, Cmd.none )

        DoNothing ->
            ( model, Cmd.none )


columnWidth : Float
columnWidth =
    80


detailsWidth =
    200


detailsHeight =
    180


arrowHeight =
    12


getLeft : Float -> Element -> Float
getLeft xClick el =
    xClick - el.element.x - detailsWidth / 2


getLeftStyle : Model -> ( Float, Float ) -> Html.Attribute msg
getLeftStyle model detailsShown =
    case model.pageRelativeReference of
        Just el ->
            style "left" (String.fromFloat (getLeft (Tuple.first detailsShown) el) ++ "px")

        Nothing ->
            style "left" "0px"


getTop : Float -> Float -> Element -> Float
getTop yClick offsetFromParent el =
    yClick - offsetFromParent - el.element.y - detailsHeight - arrowHeight


getTopStyle : Model -> ( Float, Float ) -> ( Float, Float ) -> Html.Attribute msg
getTopStyle model position offsetFromParent =
    case model.pageRelativeReference of
        Just el ->
            style "top" (String.fromFloat (getTop (Tuple.second position) (Tuple.second offsetFromParent) el) ++ "px")

        Nothing ->
            style "top" "0px"


view : Model -> Html Msg
view model =
    let
        columns =
            case model.currentYear of
                Just year ->
                    year + 2

                Nothing ->
                    0
    in
    case model.data of
        RemoteData.Loading ->
            div [] []

        RemoteData.Success wrapper ->
            let
                data =
                    case model.category of
                        Web ->
                            wrapper.software

                        _ ->
                            wrapper.wood

                sortedData =
                    data
                        |> List.sortBy (.dates >> .start)
                        |> List.reverse

                lastItem =
                    List.Extra.last sortedData

                startYear =
                    case lastItem of
                        Just techno ->
                            techno.dates.start - 1

                        Nothing ->
                            2000
            in
            div [ class model.style.class "outer-wrapper", attribute "id" "timelineWrapper" ]
                [ div []
                    [ case model.detailShown of
                        Just detailsToShow ->
                            let
                                maybeContent =
                                    case model.lang of
                                        Fr ->
                                            detailsToShow.techno.fr

                                        _ ->
                                            detailsToShow.techno.en
                            in
                            case maybeContent of
                                Just content ->
                                    div
                                        [ attribute "id" "skill-detail"
                                        , classList
                                            model.style.class
                                            [ ( "detail", True )
                                            , ( "shown", True )
                                            ]
                                        , onEnter OnDetailDivEnter
                                        , onLeave OnDetailDivLeave
                                        , getLeftStyle model detailsToShow.position
                                        , getTopStyle model detailsToShow.position detailsToShow.offsetFromParent
                                        , style "height" (String.fromFloat detailsHeight ++ "px")
                                        , style "width" (String.fromFloat detailsWidth ++ "px")
                                        ]
                                        [ div [ class model.style.class "title" ] [ text detailsToShow.techno.name ]
                                        , div [ class model.style.class "content" ] [ Markdown.toHtml [] content ]
                                        , div [ class model.style.class "footer" ]
                                            [ div [ class model.style.class "arrow" ]
                                                [ svg
                                                    [ Svg.Attributes.width "25"
                                                    , Svg.Attributes.height "12"
                                                    , Svg.Attributes.viewBox "0 0 100 50"
                                                    ]
                                                    [ Svg.polygon
                                                        [ Svg.Attributes.points "0,0 50,50 50,50 100,0"
                                                        , Svg.Attributes.fill "rgb(50,50,50)"
                                                        ]
                                                        []
                                                    , Svg.polygon
                                                        [ Svg.Attributes.points "2,-2 50,46 50,46 98,-2"
                                                        , Svg.Attributes.fill "rgb(30,30,30)"
                                                        ]
                                                        []
                                                    ]
                                                ]
                                            ]
                                        ]

                                Nothing ->
                                    div
                                        [ classList model.style.class
                                            [ ( "detail", True )
                                            , ( "shown", False )
                                            ]
                                        ]
                                        []

                        Nothing ->
                            div
                                [ classList model.style.class
                                    [ ( "detail", True )
                                    , ( "shown", False )
                                    ]
                                ]
                                []
                    ]
                , div
                    [ class model.style.class "wrapper" ]
                    [ div [ class model.style.class "title" ]
                        [ h3 []
                            [ case model.lang of
                                Fr ->
                                    text "Technologies que j'ai utilisé"

                                _ ->
                                    text "Technologies I've used"
                            ]
                        , span []
                            [ case model.lang of
                                Fr ->
                                    text "Toucher ou survoler pour voir le détail"

                                _ ->
                                    text "touch or hover to see the detail"
                            ]
                        ]
                    , div [ class model.style.class "columns-wrapper" ]
                        (List.range startYear columns
                            |> List.map
                                (\year ->
                                    div
                                        [ class model.style.class "column"
                                        , style "height" (String.fromInt (List.length data + 1) ++ "em")
                                        ]
                                        [ small [] [ text (String.fromInt year) ] ]
                                )
                        )
                    , div [ class model.style.class "skills-wrapper" ]
                        (sortedData
                            |> List.map
                                (\skill ->
                                    let
                                        left =
                                            case model.triggerAnimation of
                                                True ->
                                                    (toFloat skill.dates.start - toFloat startYear) * columnWidth

                                                False ->
                                                    0

                                        width =
                                            case model.triggerAnimation of
                                                True ->
                                                    case skill.dates.end of
                                                        Just end ->
                                                            (toFloat end - toFloat skill.dates.start) * columnWidth

                                                        Nothing ->
                                                            (toFloat columns - 2 - toFloat skill.dates.start) * columnWidth + columnWidth

                                                False ->
                                                    0
                                    in
                                    div
                                        [ class model.style.class "skill-row"
                                        , style "width" (String.fromFloat ((toFloat columns - toFloat startYear) * columnWidth + columnWidth) ++ "px")
                                        ]
                                        [ div
                                            [ class model.style.class "skill-bar"
                                            , style "width" (String.fromFloat width ++ "px")
                                            , style "left" (String.fromFloat left ++ "px")
                                            , case skill.en of
                                                Just text ->
                                                    onDown
                                                        (\event ->
                                                            ShowDetailDefer event.pagePos event.offsetPos skill
                                                        )

                                                Nothing ->
                                                    onDown (\event -> DoNothing)
                                            , case skill.en of
                                                Just text ->
                                                    onEnter
                                                        (\event ->
                                                            ShowDetailDefer event.pagePos event.offsetPos skill
                                                        )

                                                Nothing ->
                                                    onEnter (\event -> DoNothing)
                                            , onLeave ShowNothingDefer
                                            ]
                                            [ case model.triggerAnimation of
                                                True ->
                                                    span [ style "left" (String.fromFloat (width + 10) ++ "px") ]
                                                        [ text skill.name ]

                                                False ->
                                                    span [] []
                                            ]
                                        ]
                                )
                        )
                    ]
                ]

        RemoteData.Failure error ->
            div [] [ text "Http failure" ]

        RemoteData.NotAsked ->
            div [] [ text "Other" ]


port getBranch : (String -> msg) -> Sub msg


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.batch
        [ Time.every 1000 GetYear
        , case model.detailShown of
            Just details ->
                Events.onMouseDown (isOutSideDetailsTarget "skill-detail")

            Nothing ->
                Sub.none
        , Time.every 1000 GetRelativeReferenceInterval
        , case model.triggerAnimation of
            True ->
                Sub.none

            False ->
                Time.every 800 GetViewportOfComponent
        , getBranch OnNewBranch
        ]
