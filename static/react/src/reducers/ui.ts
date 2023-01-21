import _ from 'lodash';
import { UiAction } from '../actions/ui';
import { UI_UPDATE } from '../constants/ui';
import { Branch } from '../Types/index';

export interface UiState {
    BRANCH: Branch,
    PROJECT_SLUG: string | null,
}

const query = window.location.search;
const params = new URLSearchParams(query);

const branchFromString = (v: string | null): Branch => {
    switch (v) {
        case 'wood':
            return Branch.WOOD;
        case 'sofware':
        default:
            return Branch.SOFTWARE;
    }
}

const initialState: UiState = {
    BRANCH: branchFromString(params.get('domain')),
    PROJECT_SLUG: null
};

export default function ui(state: UiState = initialState, action: UiAction) {
    const newState = { ...state };
    if (action.type !== UI_UPDATE) { return state; }
    return _(newState).set(action.path, action.value).value();
}
