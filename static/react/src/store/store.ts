import { applyMiddleware, compose, createStore, Middleware } from 'redux';
// import { StoreState } from '../types/index';
import rootReducer from '../reducers/rootReducer';
const reduxDevtoolsExtensionString = '__REDUX_DEVTOOLS_EXTENSION_COMPOSE__';
import { createBrowserHistory } from 'history';

export const history = createBrowserHistory();

function configureStore(initialState?: object) {
    // compose enhancers
    const composeEnhancers =
        window[reduxDevtoolsExtensionString] && (process.env.NODE_ENV === 'development' || process.env.NODE_ENV === 'production')
            ? window[reduxDevtoolsExtensionString]({
                // options
            })
            : compose;
    const middlewares: Array<Middleware<any, any, any>> = [];

    const enhancer = composeEnhancers(applyMiddleware(...middlewares));
    // create store
    return createStore(rootReducer(history), initialState!, enhancer);
}

const store = configureStore();

export default store;
