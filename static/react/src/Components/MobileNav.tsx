import * as React from 'react';
import {
    NavLink
} from 'react-router-dom';

const MobileNav = () => (

    <nav className="mobile onlyMobile never">
        <h3 className="purple"><i className="material-icons floatLeft md-18">menu</i>Menu</h3>
        <ul>
            <li><NavLink to="/" exact={true} className="navlink" activeClassName="active">Introduction</NavLink></li>
            <li><NavLink to="/experiences" className="navlink" activeClassName="active">Expériences</NavLink></li>
            <li><NavLink to="/studies" className="navlink" activeClassName="active">Etudes</NavLink></li>
            <li><NavLink to="/skills" className="navlink" activeClassName="active">Compétences</NavLink></li>
            <li><NavLink to="/hobbies" className="navlink" activeClassName="active">Loisirs</NavLink></li>
            <li><NavLink to="/contact" className="navlink onlyMobile" activeClassName="active">Contact</NavLink></li>
        </ul>
    </nav>

);

export default MobileNav;
