import dotenv from 'dotenv';
import * as moment from 'moment';
import 'moment/locale/fr';
import * as React from 'react';
import * as ReactDOM from 'react-dom';
import { Provider } from 'react-redux';
import App from './App';
import './helpers/Sitemap';
import './index.scss';
import { language } from './internationalization/Trans';
import registerServiceWorker from './registerServiceWorker';
import store from './store/store';

dotenv.config({ path: '../.env' });

moment.locale(language);

ReactDOM.render(
    <Provider store={store}>
        <App />
    </Provider>,
    document.getElementById('root')
);

registerServiceWorker();
