import T from 'i18n-react';
const frUi = require('./fr.json');
const enUi = require('./en.json');
import en from './en';
import fr from './fr';

let language = (navigator.languages && navigator.languages[0]) ? navigator.languages[0] : navigator.language;
if (language.length > 2) {
    language = language.split('-')[0];
    language = language.split('_')[0];
}
let Dataset: any;
let uiTransJson: any;
switch (language) {
    case 'fr':
        uiTransJson = frUi;
        Dataset = fr;
        break;
    case 'en':
        uiTransJson = enUi;
        Dataset = en;
        break;
    default:
        uiTransJson = enUi;
        language = 'en';
        Dataset = en;
        break;
}


export { language };

T.setTexts(uiTransJson);

export { T };

export default Dataset;
