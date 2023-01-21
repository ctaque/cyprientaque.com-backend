module Types.Blog exposing (Msg(..))

import CssClass exposing (CssClass)
import Http
import Json.Decode as Decode exposing (Decoder, Error, Value, bool, decodeString, int, list, string)
import Json.Decode.Extra exposing (datetime)
import Json.Decode.Pipeline exposing (required, resolve)
import RemoteData exposing (WebData)
import Time
import Types.Index exposing (FilterCategory, Image, Project)


type Msg
    = GetArticlesResponse (WebData (List Project))
    | AddViewResponse (Result Http.Error String)
    | GetTimeZone Time.Zone
    | GotJwt String
    | GotSlug (Maybe String)
    | SetSlug (Maybe Project)
