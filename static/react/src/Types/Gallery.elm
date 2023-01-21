module Types.Gallery exposing (Msg(..))

import Http
import Keyboard exposing (RawKey)
import RemoteData exposing (WebData)
import Time
import Types.Index
    exposing
        ( AccessToken
        , BitbucketProject
        , Category
        , FilterCategory
        , Image
        , ImgDimensions
        , Project
        , RepositoryListResponse
        , WheelDirection
        )


type Msg
    = GetImagesResponse (WebData (List Image))
    | AddImageViewResponse (Result Http.Error String)
    | GotJwt String
    | FadeIn Time.Posix
    | SetActiveImage ( Maybe Image )
    | GetProjectResponse (WebData Project)
    | DoNothing
    | GotBranch String

