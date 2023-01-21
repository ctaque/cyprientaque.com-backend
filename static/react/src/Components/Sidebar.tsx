import * as React from 'react';
import { push as Menu, State } from 'react-burger-menu';
import { useLocation } from 'react-router-dom';
import { Link } from 'react-scroll';
import { T } from '../internationalization/Trans';
import SidebarStyles from './SidebarStyle';

const sidebar = () => {

    const [isOpen, setIsOpen] = React.useState(false);

    const location = useLocation();

    React.useEffect(() => {
        setIsOpen(false);
    }, [location]);

    const onStateChange = (state: State) => {
        setIsOpen(state.isOpen);
    };

    return (
        <Menu right={true}
            isOpen={isOpen}
            width={200}
            styles={SidebarStyles}
            burgerButtonClassName={'onlyMobile'}
            outerContainerId={'outer-container'}
            onStateChange={onStateChange}
        >
            <h3 className="">Menu</h3>
            <ul className="sidebar-list">
                <li>
                    <Link to="intro" smooth={true} activeClass="active" spy={true} hashSpy={true}>
                        <span>{T.translate('home.shortMenuTitle')}</span>
                    </Link>
                </li>
                <li>
                    <Link to="experience" smooth={true} activeClass="active" spy={true} hashSpy={true}>
                        <span>{T.translate('experiences.shortMenuTitle')}</span>
                    </Link>
                </li>
                <li>
                    <Link to="studies" smooth={true} activeClass="active" spy={true} hashSpy={true}>
                        <span>{T.translate('studies.shortMenuTitle')}</span>
                    </Link>
                </li>
                <li>
                    <Link to="skills" smooth={true} activeClass="active" spy={true} hashSpy={true}>
                        <span>{T.translate('skills.shortMenuTitle')}</span>
                    </Link>
                </li>
                <li>
                    <Link to="hobbies" smooth={true} activeClass="active" spy={true} hashSpy={true}>
                        <span>{T.translate('hobbies.shortMenuTitle')}</span>
                    </Link>
                </li>
                <li>
                    <Link to="portfolio" smooth={true} activeClass="active" spy={true} hashSpy={true}>
                        <span>{T.translate('portfolio.shortMenuTitle')}</span>
                    </Link>
                </li>
                <li>
                    <a href="https://ctprods.cyprientaque.com/blog">
                        Blog
                    </a>
                </li>
                <li style={{ display: 'none'}}>
                    <Link to="contact" smooth={true} activeClass="active" spy={true} hashSpy={true}>
                        <span>{T.translate('portfolio.shortMenuTitle')}</span>
                    </Link>
                </li>
            </ul>
        </Menu>
    );
};

export default sidebar;
