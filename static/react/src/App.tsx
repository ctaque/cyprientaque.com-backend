import * as React from 'react';
import Home from './Components/Home';
import ToggleSwitch from './Components/ToggleSwitch';
import { T } from './internationalization/Trans';

const App = () => {
    return (
        <div className="AppWrapper" id="outer-container" >
            <div className="toggleSwitchLabelWrapper">
                <span>{T.translate('header.switchLabel.software')}</span>
                <ToggleSwitch />
                <span>{T.translate('header.switchLabel.wood')}</span>
            </div>
            <main id="pageWrap">
                <div className="PageWrap">
                    <Home />
                </div>
            </main>
        </div>
    );
};
export default App;
