module Types.Portfolio exposing (GetProjectsResponseNewData, Msg(..), OnMouseWheelNextData)

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
    = GetProjectsResponse (WebData (List Project))
    | SetCategory FilterCategory
    | SetCurrentProject (Maybe Project)
    | SetCurrentImage (Maybe Image)
    | AddViewResponse (Result Http.Error String)
    | AddImageViewResponse (Result Http.Error String)
    | ImgLoaded ImgDimensions
    | SetPrimaryImage Project
    | CycleImages Project Image
    | PlayCarousel
    | PauseCarousel
    | OnTime Time.Posix
    | GotJwt String
    | GotSlug ( Maybe String )
    | GetRepositoriesResponse (WebData RepositoryListResponse)
    | GetAccessTokenResponse (WebData AccessToken)
    | GetBitbucketProjectResponse (WebData BitbucketProject)
    | OnMouseWheel Project (List Project) WheelDirection
    | SetScrollTimer Time.Posix
    | OnKeyDown RawKey
    | GotBranch String
    | GetCategoriesResponse (WebData (List Category))
    | SetRealCategory Category
    | HandleSearch String
    | SearchProjectsResponse (WebData (List Int))
    | ClearSearchInput


type alias OnMouseWheelNextData =
    { project : Project
    , image : Maybe Image
    , fetchBitbucketProjectCmd : Cmd Msg
    }


type alias GetProjectsResponseNewData =
    { project : Maybe Project
    , image : Maybe Image
    , addViewCmd : Cmd Msg
    , category : FilterCategory
    }
