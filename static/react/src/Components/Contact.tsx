import * as moment from 'moment';
import * as React from 'react';
import { Helmet } from 'react-helmet';
import { JsonLd } from "react-schemaorg";
import { CreativeWork } from 'schema-dts';

const Contact = () => {
    return (
        <div className="contactSection">
            <Helmet>
                <title>Contact | Cyprien Taque</title>
            </Helmet>

            <JsonLd<CreativeWork> item={{
                "@context": "https://schema.org",
                "@type": "CreativeWork",
                "name": "Contact me",
                "url": window.location.href
            }} />
            <h2 className="onlyMobile purple regular">Contact</h2>
            <ul>
                <li className="grey">Cyprien Taque</li>
                <li className="purple"><a href="mailto:cyprien.taque@gmail.com">cyprien.taque@gmail.com</a></li>
                <li className="purple">07 82 68 61 16</li>
                <li className="purple">Nantes 44300</li>
                <li><a href="https://bitbucket.com/syprex" target="_blank">bitbucket</a></li>
                <li><a href="https://s3-eu-west-1.amazonaws.com/ctaque.logos/cv_Cyprien_Taque.pdf" target="_blank" >CV.pdf</a></li>
                <li className="purple">{moment().diff(moment('1990-05-05'), 'years')} ans</li>
            </ul>
        </div>
    )
};

export default Contact;
