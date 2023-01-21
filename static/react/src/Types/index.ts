import { UiState } from "../reducers/ui";

export enum Branch {
    SOFTWARE = 'software',
    WOOD = 'wood'
}

export interface StoreState {
    ui: UiState
}
