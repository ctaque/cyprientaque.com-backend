import * as React from 'react';
import { useLocation } from 'react-router-dom';
import { Link } from 'react-scroll';
import { T } from '../internationalization/Trans';
import ArrowNavLinks from './ArrowNavLinks';
import ToggleSwitch from './ToggleSwitch';

const hashs = [
    'intro',
    'experience',
    'studies',
    'skills',
    'hobbies',
    'portfolio',
    'blog'
];

const matchIndex = (currentRoute: string): number | null => hashs.reduce(
    (accu: number | null, hash: string, index: number) => {
        if (currentRoute === hash) {
            return index;
        }
        return accu;
    }
    , null
);

const isElementInViewport = (el: HTMLElement) => {

    const rect = el.getBoundingClientRect();
    return (
        rect.top >= -el.offsetHeight &&
        rect.bottom <= (window.innerHeight || document.documentElement.clientHeight)
    );
}


window.addEventListener('scroll', () => {
    const headerEl = document.getElementById('main-header');
    const navEl = document.getElementById('desktop-nav-fixed');
    if (!headerEl || !navEl) {
        return;
    } else {
        if (isElementInViewport(headerEl)) {
            navEl.classList.add('hidden');
        } else {
            navEl.classList.remove('hidden');
        }
    }
});

interface Props {
    id: string;
    classNames: string[];
}

const offsetLeft = 950 / 7;

const getClipPath = (offset: number | null) => {
    return `circle(5px at calc((100% - 950px) / 2 + 10px + ${offset}px)  50%)`;
}


const DesktopNav = (props: Props) => {
    const location = useLocation();
    const [offsetFromLeftPx, setOffsetLeft] = React.useState((matchIndex(location.hash) || 0) * offsetLeft);
    const [currentHash, setCurrentHash] = React.useState(location.hash);
    React.useEffect(() => {
        const nextOffset = (matchIndex(currentHash) || 0) * offsetLeft;
        const mustUpdate = nextOffset !== offsetFromLeftPx;
        if (mustUpdate) {
            setOffsetLeft(nextOffset);
        }
    }, [currentHash]);
    return (
        <nav
            id={props.id}
            className={props.classNames.join(' ')}
            key={window.location.pathname}
        >
            <div className="toggleSwitchLabelWrapper">
                <span>{T.translate('header.switchLabel.software')}</span>
                <ToggleSwitch location={location} />
                <span>{T.translate('header.switchLabel.wood')}</span>
            </div>
            <div className="bg onlyDesktop" style={{ clipPath: getClipPath(offsetFromLeftPx) }} />
            <ArrowNavLinks.NextLinks />
            <ArrowNavLinks.PrevLinks />
            <div className="PageWrap onlyDesktop">
                <ul>
                    <li>
                        <Link to="intro" smooth={true} activeClass="active" spy={true} hashSpy={true} onSetActive={setCurrentHash}>
                            <span>{T.translate('home.shortMenuTitle')}</span>
                        </Link>
                    </li>
                    <li>
                        <Link to="experience" smooth={true} activeClass="active" spy={true} hashSpy={true} onSetActive={setCurrentHash}>
                            <span>{T.translate('experiences.shortMenuTitle')}</span>
                        </Link>
                    </li>
                    <li>
                        <Link to="studies" smooth={true} activeClass="active" spy={true} hashSpy={true} onSetActive={setCurrentHash}>
                            <span>{T.translate('studies.shortMenuTitle')}</span>
                        </Link>
                    </li>
                    <li>
                        <Link to="skills" smooth={true} activeClass="active" spy={true} hashSpy={true} onSetActive={setCurrentHash}>
                            <span>{T.translate('skills.shortMenuTitle')}</span>
                        </Link>
                    </li>
                    <li>
                        <Link to="hobbies" smooth={true} activeClass="active" spy={true} hashSpy={true} onSetActive={setCurrentHash}>
                            <span>{T.translate('hobbies.shortMenuTitle')}</span>
                        </Link>
                    </li>
                    <li>
                        <Link to="portfolio" smooth={true} activeClass="active" spy={true} hashSpy={true} onSetActive={setCurrentHash}>
                            <span>{T.translate('portfolio.shortMenuTitle')}</span>
                        </Link>
                    </li>
                    <li>
                        <a href="https://ctprods.cyprientaque.com/blog">
                            <span style={{ display: 'flex', flexDirection: 'row', alignItems: 'center', justifyContent: 'center' }}>
                                <svg viewBox="0 0 1025 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="2051" width="15" height="15">
                                    <path d="M384 0l0 96c73.472 0 144.704 14.368 211.712 42.72 64.768 27.392 122.944 66.624 172.96 116.608s89.216 108.192 116.64 172.96c28.352 67.008 42.72 138.24 42.72 211.712l96 0c0-353.472-286.528-640-640-640z" p-id="2052" fill="#fff" />
                                    <path d="M384 192l0 96c94.016 0 182.432 36.608 248.896 103.104s103.104 154.88 103.104 248.896l96 0c0-247.424-200.576-448-448-448z" p-id="2053" fill="#fff" />
                                    <path d="M480 384l-64 64-224 64-192 416 25.376 25.376 232.8-232.8c-1.408-5.28-2.176-10.848-2.176-16.576 0-35.36 28.64-64 64-64s64 28.64 64 64-28.64 64-64 64c-5.728 0-11.296-0.768-16.576-2.176l-232.8 232.8 25.376 25.376 416-192 64-224 64-64-160-160z" p-id="2054" fill="#fff" />
                                </svg> &nbsp;
                            Blog
                            </span>
                        </a>
                    </li>
                </ul>
            </div>
        </nav>
    );
}


export default DesktopNav;
