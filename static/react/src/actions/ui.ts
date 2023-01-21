import {
    PROJECT_SLUG, SWITCH_BRANCH,
    UI_UPDATE
} from '../constants/ui';
import { Branch } from '../Types';

interface Action {
    type: UI_UPDATE;
    path: string;
}

export interface SwitchBranch extends Action {
    value: Branch;
}

export const switchBranch = (branch: Branch): SwitchBranch => ({
    type: UI_UPDATE,
    path: SWITCH_BRANCH,
    value: branch
});

export interface SetProjectslug extends Action {
    value: string | null;
}

export const setProjectSlug = (slug: string | null): SetProjectslug => ({
    type: UI_UPDATE,
    path: PROJECT_SLUG,
    value: slug,
})

export type UiAction = SwitchBranch | SetProjectslug;
