import * as React from 'react';
import { Helmet } from 'react-helmet';
import Home from './Components/Home';

const App = () => {
    return (
        <div className="AppWrapper" id="outer-container" >
            <Helmet>
                <title>Cv de Cyprien Taque </title>
                <meta name="description" content="Cv interactif de Cyprien Taque. Ici vous trouverez de l'informatique, de la CAO et du travail du bois" />
            </Helmet>
            <main id="pageWrap">
                <div className="PageWrap">
                    <Home />
                </div>
            </main>
        </div>
    );
};
export default App;
