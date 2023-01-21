import * as React from 'react';
import { Link } from 'react-scroll';
import { Transition } from 'react-transition-group';
import { T } from '../internationalization/Trans';

const nextLinks = {
    root: {
        anchor: 'experiences',
        text: T.translate('experiences.shortMenuTitle')

    },
    experiences: {
        anchor: 'studies',
        text: T.translate('studies.shortMenuTitle')
    },
    studies: {
        anchor: 'skills',
        text: T.translate('skills.shortMenuTitle')
    },
    skills: {
        anchor: 'hobbies',
        text: T.translate('hobbies.shortMenuTitle')
    },
    hobbies: {
        anchor: 'portfolio',
        text: T.translate('portfolio.shortMenuTitle')
    },
    portfolio: {
        anchor: 'blog',
        text: 'Blog'
    }
}

const prevLinks = {
    blog: {
        anchor: 'portfolio',
        text: T.translate('hobbies.shortMenuTitle')
    },
    portfolio: {
        anchor: 'hobbies',
        text: T.translate('hobbies.shortMenuTitle')
    },
    hobbies: {
        anchor: 'skills',
        text: T.translate('skills.shortMenuTitle')
    },
    skills: {
        anchor: 'studies',
        text: T.translate('studies.shortMenuTitle')
    },
    studies: {
        anchor: 'experiences',
        text: T.translate('experiences.shortMenuTitle')
    },
    experiences: {
        anchor: 'intro',
        text: T.translate('home.shortMenuTitle')
    }
}

const duration = 100;

const defaultStyle = {
    transition: `opacity ${duration}ms ease-in-out`,
    opacity: 0,
}

const transitionStyles = {
    entering: { opacity: 0 },
    entered: { opacity: 1 },
    exiting: { opacity: 0 },
    exited: { opacity: 0 }
};

const Curried = ({ path, links, icon, className }: any) => (
    <Transition key={Math.random() * 1000} unmountOnExit={true} appear={true} exit={false} in={true} timeout={{ enter: 300, exit: 100 }} >
        {(status: string) => (
            <div className={`arrowNavLink arrowNavLink-${className}`} style={{ ...defaultStyle, ...transitionStyles[status] }}>
                {icon}
                <Link to={`${links[path].anchor}`}>{links[path].text}</Link>
            </div>
        )}
    </Transition>
);

const ArrowNavLinks = {
    NextLinks: (props: any) => {
        const path = window.location.hash || 'intro';
        return (
            nextLinks[path]
                ? <Curried
                    path={path}
                    links={nextLinks}
                    icon={<i className="material-icons icon-nextLink">keyboard_arrow_right</i>}
                    className="nextLink"
                />
                : <div />
        );
    },
    PrevLinks: (props: any) => {
        const path = window.location.pathname.split('/')[1] || 'root';
        return (
            prevLinks[path]
                ? <Curried
                    path={path}
                    links={prevLinks}
                    icon={<i className="material-icons icon-prevLink">keyboard_arrow_left</i>}
                    className="prevLink" />
                : <div />
        );
    }
}

export default ArrowNavLinks;
