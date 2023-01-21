module Elm.Lib.Functions exposing (errorToString, firstIndexOf, lastIndexOf)

import Http exposing (Error(..))
import Json.Decode as Decode
import Json.Encode as Encode
import JsonWebToken as JWT exposing (hmacSha256)
import Types.Index exposing (Claims)


indicesOf : a -> List a -> List Int
indicesOf thing things =
    things
        |> List.indexedMap Tuple.pair
        |> List.filter (\( idx, item ) -> item == thing)
        |> List.map Tuple.first


firstIndexOf : a -> List a -> Int
firstIndexOf thing things =
    indicesOf thing things
        |> List.minimum
        |> Maybe.withDefault -1


lastIndexOf : a -> List a -> Int
lastIndexOf thing things =
    indicesOf thing things
        |> List.maximum
        |> Maybe.withDefault -1


errorToString : Http.Error -> String
errorToString err =
    case err of
        Timeout ->
            "Timeout exceeded"

        NetworkError ->
            "Network error"

        BadStatus status ->
            "Unexpected Status: " ++ String.fromInt status

        BadBody text ->
            "Unexpected response from api: " ++ text

        BadUrl url ->
            "Malformed url: " ++ url


encodeClaims : Claims -> Encode.Value
encodeClaims claims =
    Encode.object
        [ ( "iat", Encode.int claims.iat )
        , ( "exp", Encode.int claims.exp )
        , ( "user_id", Encode.int claims.user_id )
        ]
