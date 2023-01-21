import { History } from 'history';
import { combineReducers } from 'redux';

import ui from './ui';

const rootReducer = (history: History) => combineReducers({
    ui
});

export default rootReducer;
