import * as React from 'react';
import Home from './Components/Home';

const App = () => {
    return (
        <div className="AppWrapper" id="outer-container" >
            <main id="pageWrap">
                <div className="PageWrap">
                    <Home />
                </div>
            </main>
        </div>
    );
};
export default App;
