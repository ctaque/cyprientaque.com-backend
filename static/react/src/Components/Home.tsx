import gsap from 'gsap';
import ScrollTrigger from 'gsap/ScrollTrigger';
import JWT from 'jsonwebtoken';
import * as React from 'react';
import Elm from 'react-elm-components';
import { Helmet } from 'react-helmet';
import ReactMarkdown from 'react-markdown/with-html';
import Moment from 'react-moment';
import { connect, ConnectedProps } from 'react-redux';
import { JsonLd } from "react-schemaorg";
import { Dispatch } from 'redux';
import { CreativeWork } from 'schema-dts';
import { addMarkToMarkDown, getHighlightKeyword, HighlightedText } from 'src/helpers/func';
import { Branch, StoreState } from 'src/Types';
import * as uiActions from '../actions/ui';
import Portfolio from '../Elm/Components/Portfolio.elm';
import SkillTimeLine from '../Elm/Components/Skills.elm';
import Dataset, { language, T } from '../internationalization/Trans';
import DOOM_ICON from '../logo-doom-emacs.png';
import store from '../store/store';

const skillsStyle = require('../Elm/Components/Skills.modules.scss');
const portfolioStyle = require('../Elm/Components/Portfolio.modules.scss');

(window as any).ga('send', 'pageview');

interface Props {
    images: CarouselData[];
    imagekey: string;
    branch: Branch;
    onDiscoverBtnClick: (current: string) => () => void;
}

interface State {
    idx: number;
}

const getToken = () => JWT.sign({
    "iat": new Date().getTime(),
    "user_id": 1,
    "exp": Math.round(
        (new Date().getTime() + 10 * 1000 /* milliseconds */) / 1000
        /* seconds */) // 10 s
}, process.env.REACT_APP_JWT_SECRET || '');

class Caroussel extends React.Component<Props, State> {
    public interval: number;
    constructor(props: Props) {
        super(props);
        this.state = {
            idx: 0,
        };
        this.play = this.play.bind(this);
        this.pause = this.pause.bind(this);
        this.handleMouseEnter = this.handleMouseEnter.bind(this);
        this.handleMouseLeave = this.handleMouseLeave.bind(this);
        this.setIndex = this.setIndex.bind(this);
    }

    public handleMouseEnter(e: React.MouseEvent<HTMLImageElement>) {
        this.pause();
    }

    public handleMouseLeave(e: React.MouseEvent<HTMLImageElement>) {
        this.play();
    }
    public play() {
        this.interval = window.setInterval(
            () => {
                const idxRef = this.state.idx;
                this.setState({
                    idx: (
                        idxRef === this.props.images.length - 1
                            ? 0
                            : idxRef + 1
                    )
                });
            }
            , 3000
        );
    }

    public pause() {
        window.clearInterval(this.interval);
    }

    public setIndex(idx: number) {
        return (e: React.MouseEvent<HTMLButtonElement>) => {
            this.pause();
            this.setState({
                idx
            });
        }
    }

    public componentWillReceiveProps(nextProps: Props) {
        if (nextProps.branch !== this.props.branch) {
            // on réinitialise à 0
            this.setState({
                idx: 0
            });
        }
    }

    public componentWillMount() {
        this.play();
    }

    public componentWillUnmount() {
        this.pause();
    }

    public render() {
        return (
            <div className="carousselWrapper">
                <div className="imagesWrapper" style={{ left: `-${this.state.idx * 100}%` }}>
                    {this.props.images.map((current: CarouselData, i: number) =>
                        <React.Fragment key={`${current.url}-${this.props.imagekey}`}>
                            <img
                                onMouseEnter={this.handleMouseEnter}
                                onMouseLeave={this.handleMouseLeave}
                                src={current.url}
                                style={{ left: `${i * 100}%` }}
                            />
                            <div className="cta" style={{ left: `${i * 100}%` }}>
                                <button onClick={this.props.onDiscoverBtnClick(current.slug)} style={{ backgroundColor: '#ffeb3b', borderRadius: '5rem', padding: 0, lineHeight: 0, border: 'none' }}>
                                    <svg
                                        viewBox="0 0 1024 1024"
                                        width="40"
                                        height="40"
                                    >
                                        <path
                                            d="M512 64q190.016 4.992 316.512 131.488T960 512q-4.992 190.016-131.488 316.512T512 960q-190.016-4.992-316.512-131.488T64 512q4.992-190.016 131.488-316.512T512 64z m67.008 275.008q26.016 0 43.008-15.488t16.992-41.504-16.992-41.504-42.496-15.488-42.496 15.488-16.992 41.504 16.992 41.504 42.016 15.488z m12 360q0-6.016 0.992-16t0-19.008l-52.992 60.992q-8 8.992-16.512 14.016t-14.496 3.008q-8.992-4-8-14.016l88-276.992q4.992-28-8.992-48t-44.992-24q-35.008 0.992-76.512 29.504t-72.512 72.512v15.008q-0.992 10.016 0 19.008l52.992-60.992q8-8.992 16.512-14.016t13.504-3.008q10.016 4.992 7.008 16l-87.008 276q-7.008 24.992 7.008 44.512t48.992 26.496q50.016-0.992 84-28.992t63.008-72z"
                                            fill="#555"
                                        />
                                    </svg>
                                </button>
                            </div>
                        </React.Fragment>
                    )}
                </div>
                <div className="controlsWrapper">
                    {
                        this.props.images.map((current: CarouselData, i: number) =>
                            <button
                                key={current.url}
                                onClick={this.setIndex(i)}
                                className={[
                                    'dot',
                                    this.props.images[this.state.idx].url === current.url
                                        ? 'active'
                                        : ''
                                ].join(' ')}
                            />
                        )
                    }
                </div>
            </div>
        );
    }
}

interface CarouselData {
    url: string,
    slug: string,
};

const imagesWeb: CarouselData[] = [{
    url: 'https://s3.eu-west-2.amazonaws.com/ctprods-laravel-production/projectsImages/47/w1500-59de52810f6c42dbb608dee55b7f38d1.png',
    slug: 'cli-headless-cms'
}, {
    url: 'https://ctprods-laravel-production.s3.eu-west-2.amazonaws.com/projectsImages/22/w1500-tfAR7vPD78vdnyh1EBgZPQTvkLiawIntE6oVkOmR.jpeg',
    slug: 'cine-boutique'
}, {
    url: 'https://ctprods-laravel-production.s3.eu-west-2.amazonaws.com/projectsImages/21/w1500-5V775N3Y2hsANivEqx6FBP9VtwAm8Ttjt18CuJ6I.png',
    slug: 'station-meteo'
}];

const imagesWood: CarouselData[] = [{
    url: 'https://s3.eu-west-2.amazonaws.com/ctprods-laravel-production/projectsImages/13/w1500-afF0lQKRcyfOe5AJZHh7zCRoWIPxAsGGWWnMtuv9.jpeg',
    slug: 'table-de-chevet'
}, {
    url: 'https://s3.eu-west-2.amazonaws.com/ctprods-laravel-production/projectsImages/11/w1500-vCEvf05UxXd75V9H61G5RyCxmfauWIqCfxFVr4Kf.jpeg',
    slug: 'modele-reduit-de-classe-america'
}, {
    url: 'https://s3.eu-west-2.amazonaws.com/ctprods-laravel-production/projectsImages/11/w1500-a97rzjvbzeCD64nhLWpb2N8CC80RyAPcDx8hIG6p.jpeg',
    slug: 'modele-reduit-de-classe-america'
}, {
    url: 'https://ctprods-laravel-production.s3.eu-west-2.amazonaws.com/projectsImages/6/w1500-WgAQ5xOIkwMr2WDhTrsttPZEuTOlSexegeLULCEz.jpeg',
    slug: 'banquettes-petales-en-corian-et-noyer'
}];

const mapStateProps = (localStore: StoreState) => ({
    branch: localStore.ui.BRANCH,
    projectSlug: localStore.ui.PROJECT_SLUG,
});

const mapDispatchProps = (dispatch: Dispatch) => ({
    setBranch: (value: Branch) => dispatch(uiActions.switchBranch(value)),
    setProjectSlug: (value: string | null) => dispatch(uiActions.setProjectSlug(value)),
});

const connector = connect(mapStateProps, mapDispatchProps);

type PropsFromRedux = ConnectedProps<typeof connector>;

type HomeProps = PropsFromRedux;
let skillPreviousValue: string | null = null;
let portfolioPreviousValue: string | null = null;
let portFolioPreviousSlug: string | null = null;

export default connector(
    (props: HomeProps) => {
        const [filter, setFilter] = React.useState<string | undefined>(undefined);
        const introRef = React.useRef(null);
        const experienceRef = React.useRef(null);
        const skillsRef = React.useRef(null);
        const hobbiesRef = React.useRef(null);
        const portfolioRef = React.useRef(null);
        const studiesRef = React.useRef(null);
        const doSetFilter = (current: string | undefined) => () => {
            setFilter(current);
        };

        const buildAnimation = () => {
            /* gsap.from(introRef.current, {
             *     scrollTrigger: {
             *         trigger: '#intro',
             *         start: 'top 50%',
             *         end: 'top 30%',
             *         scrub: true,
             *         toggleActions: "play reverse play reverse",
             *     },
             *     x: -100,
             *     opacity: 0,
             * }); */
            const isMobile = window.innerWidth < 767;
            const fromRight = () => isMobile ? 0 : 100;
            const fromLeft = () => isMobile ? 0 : -100;
            gsap.from(experienceRef.current, {
                scrollTrigger: {
                    trigger: '#experience',
                    start: 'top 50%',
                    end: 'top 30%',
                    scrub: true,
                    /* toggleActions: "play reverse play reverse", */

                },
                x: fromRight(),
                opacity: 0,
            });
            gsap.from(studiesRef.current, {
                scrollTrigger: {
                    trigger: '#studies',
                    start: 'top 50%',
                    end: 'top 30%',
                    scrub: true,
                    /* toggleActions: "play reverse play reverse", */
                },
                x: fromLeft(),
                opacity: 0,
            });
            gsap.from(skillsRef.current, {
                scrollTrigger: {
                    trigger: '#skills',
                    start: 'top 50%',
                    end: 'top 30%',
                    scrub: true,
                    /* toggleActions: "play reverse play reverse", */
                },
                x: fromRight(),
                opacity: 0,
            });
            gsap.from(hobbiesRef.current, {
                scrollTrigger: {
                    trigger: '#hobbies',
                    start: 'top 50%',
                    end: 'top 30%',
                    scrub: true,
                    /* toggleActions: "play reverse play reverse", */
                },
                x: fromLeft(),
                opacity: 0,
            });
        };

        function usePrevious(value: Branch) {
            const ref = React.useRef<Branch>();
            React.useEffect(() => {
                ref.current = value;
            });
            return ref.current;
        }
        const previousBranch = usePrevious(props.branch);
        React.useEffect(
            () => {
                if (previousBranch !== props.branch && previousBranch !== undefined) {
                    gsap.killTweensOf('#experience');
                    gsap.killTweensOf('#studies');
                    gsap.killTweensOf('#skills');
                    gsap.killTweensOf('#hobbies');
                    gsap.set('#experience', { x: 0, opacity: 1 });
                    gsap.set('#studies', { x: 0, opacity: 1 });
                    gsap.set('#skills', { x: 0, opacity: 1 });
                    gsap.set('#hobbies', { x: 0, opacity: 1 });
                } else {
                    gsap.registerPlugin(ScrollTrigger);
                    buildAnimation();
                }
            },
            [props.branch]
        );

        function selectBranch(state: StoreState) {
            return state.ui.BRANCH;
        }

        function setupSkillsPorts(ports: any) {
            let currentValue = props.branch;
            const handleChange = () => {
                currentValue = selectBranch(store.getState());

                if (skillPreviousValue !== currentValue) {
                    skillPreviousValue = currentValue;
                    ports.getBranch.send(currentValue);
                }
            }
            const unsubscribeSkillPorts = store.subscribe(handleChange)
            return () => {
                unsubscribeSkillPorts();
            }
        }

        const doSetProjectSlug = (current: string) => {
            return () => {
                props.setProjectSlug(current);
            }
        };

        function selectProjectSlug(state: StoreState) {
            return state.ui.PROJECT_SLUG;
        }

        function setupPortfolioPorts(ports: any) {
            window.setInterval(() => {
                ports.getJwt.send(getToken());
            }, 8000);

            ports.setBranch.subscribe((newBranch: string) => {
                props.setBranch(newBranch === 'software' ? Branch.SOFTWARE : Branch.WOOD)
            });
            let currentValue = props.branch;
            const handleChange = () => {
                currentValue = selectBranch(store.getState());

                if (portfolioPreviousValue !== currentValue) {
                    portfolioPreviousValue = currentValue;
                    ports.getBranch.send(currentValue);
                }
            }

            const unsubscribePorfolioPorts = store.subscribe(handleChange)

            let currentSlug = props.projectSlug;
            const handleChangeSlug = () => {
                currentSlug = selectProjectSlug(store.getState());

                if (portFolioPreviousSlug !== currentSlug) {
                    portFolioPreviousSlug = currentSlug;
                    ports.getSlug.send(currentSlug);
                }
            }
            const unsubscribeProjectSlugPort = store.subscribe(handleChangeSlug);

            return () => {
                unsubscribePorfolioPorts();
                unsubscribeProjectSlugPort();
            }
        }

        const env = process.env.NODE_ENV;
        const jwtToken = getToken();
        const { branch } = props;
        const datasetBranch = branch === Branch.WOOD ? 'wood' : 'software';
        const dataExp = Dataset.experiences[branch].filter((value: any) => filter ? value.contract_type === filter : true);
        const dataStudies = Dataset.studies[branch];
        const dataSkills = Dataset.skills[branch];
        const hl = getHighlightKeyword(location.search);
        const slug = new URLSearchParams(location.search).get('p');
        return (
            <React.Fragment>
                <Helmet>
                    <title>Cyprien Taque</title>
                </Helmet>

                <JsonLd<CreativeWork> item={{
                    "@context": "https://schema.org",
                    "@type": "CreativeWork",
                    "name": "Introduction",
                    "description": Dataset.home.intro,
                    "url": window.location.href
                }} />
                <br />
                <div className="section" id="intro" ref={introRef}>
                    <h1 className="regular purple">{Dataset.home.about}</h1>
                    <HighlightedText text={Dataset.home.intro} highlight={hl} className="grey fs15" />
                    <br />
                    <div className="home-content-wrapper">
                        <div className="caroussel">
                            <Caroussel
                                images={branch === Branch.WOOD ? imagesWood : imagesWeb}
                                imagekey={branch === Branch.WOOD ? "wood" : "web"}
                                branch={branch}
                                onDiscoverBtnClick={doSetProjectSlug}
                            />
                        </div>
                        <h2 className="regular purple">{Dataset.home[datasetBranch].title}</h2>
                        <ReactMarkdown
                            className="grey markdown fs12"
                            source={addMarkToMarkDown(Dataset.home[datasetBranch].introduction, hl)}
                            allowDangerousHtml={true}
                        />
                        <div className="clearfix" />
                    </div>
                </div>
                <div className="spacer" />
                <div className="section" id="experience" ref={experienceRef}>
                    <h1 className="purple regular subTitle">{T.translate(`experiences.mainTitle.${branch}`)}</h1>
                    <p className="noMargin">
                        <input name="filter" id="all" onChange={doSetFilter(undefined)} defaultChecked={true} type="radio" value="" />
                        <label htmlFor="all" className="grey block fs12">{T.translate('experiences.checkbox.all')}</label>
                    </p>
                    <p className="noMargin">
                        <input name="filter" id="cdi" onChange={doSetFilter('travail')} type="radio" value="" />
                        <label htmlFor="cdi" className="grey block fs12">{T.translate('experiences.checkbox.contract')}</label>
                    </p>
                    <p className="noMargin">
                        <input name="filter" id="stage" onChange={doSetFilter('stage')} type="radio" value="" />
                        <label htmlFor="stage" className="grey block fs12">{T.translate('experiences.checkbox.internship')}</label>
                    </p>
                    <div className="timeline filterContainer">
                        {
                            dataExp.map((unit: any) =>
                                <ul key={unit.id} className={['relative experienceUnit mainList-item', unit.contract_type].join(' ')}>
                                    <HighlightedText text={unit.institution} highlight={hl} className="purple timelineItem fs18" />
                                    <span className="timelineItem-date">
                                        <Moment parse="MM/YYYY" fromNow={true} ago={true}>
                                            {unit.beginning}
                                        </Moment>
                                    </span>
                                    {unit.icon
                                        ? <div className="technologyIcon">
                                            <img alt="icon" src={unit.icon} />
                                        </div>
                                        : ''
                                    }
                                    <span className="grey light fs12">{` - ${unit.location} - `}</span>
                                    <span className="grey light fs12">
                                        <Moment parse="MM/YYYY" format="YYYY">
                                            {unit.beginning}
                                        </Moment>
                                        {unit.end
                                            ? ''
                                            : ' - en cours'
                                        }
                                    </span>
                                    <span className="regular purple fs15" style={{ display: 'block', marginTop: '.7rem' }}>{unit.headline}</span>
                                    <ul>
                                        <li className="grey">
                                            <ReactMarkdown
                                                source={addMarkToMarkDown(unit.tasks, hl)}
                                                allowDangerousHtml={true}
                                                className="fs12"
                                            />
                                        </li>
                                        <li className="grey">
                                            <span className="regular purple fs12">Technologies: </span>
                                            <HighlightedText text={unit.technologies} highlight={hl} className="purple fs12" />
                                        </li>
                                        <li className="grey">
                                            <span className="regular purple fs12">Environnement: </span>
                                            <HighlightedText text={unit.environment} highlight={hl} className="purple fs12" />
                                        </li>
                                    </ul>
                                </ul>
                            )
                        }
                    </div>
                </div>
                <div className="spacer" />
                <div className="section" id="studies" ref={studiesRef}>
                    <h1 className="purple regular subTitle">{T.translate(`studies.mainTitle.${branch}`)}</h1>
                    {
                        dataStudies.map((unit: any) =>
                            (
                                <ul key={unit.id} className="relative mainList-item">
                                    <li>
                                        {unit.icon
                                            ? <div className="technologyIcon">
                                                <img alt="icon" src={unit.icon} />
                                            </div>
                                            : ''
                                        }
                                        <strong className="purple">
                                            <HighlightedText text={unit.institution} highlight={hl} className="fs18" />
                                        </strong>
                                        <span className="grey light fs15" style={{ marginLeft: '.5rem' }}>
                                            {unit.location}
                                            <span style={{ marginLeft: '.5rem' }}>
                                                <Moment parse="MM/YYYY" format="YYYY">
                                                    {unit.beginning}
                                                </Moment>
                                            </span>
                                                -
                                            <span>
                                                <Moment parse="MM/YYYY" format="YYYY">
                                                    {unit.end}
                                                </Moment>
                                            </span>
                                        </span><br />
                                        <span className="grey fs12" key={unit.id}>{unit.text}</span>
                                        <span className="grey block fs12" style={{ marginTop: '.7em' }} key={unit.id}>
                                            <HighlightedText text={unit.courses} highlight={hl} className="fs12" />
                                        </span>
                                    </li>
                                </ul>
                            )
                        )
                    }
                </div>
                <div className="spacer" />
                <div className="section" id="skills" ref={skillsRef}>
                    <h1 className="purple regular subTitle">{T.translate(`skills.mainTitle.${branch}`)}</h1>
                    {
                        dataSkills.map((unit: any) =>
                            (
                                <ul key={unit.id} className="mainList-item">
                                    <li className="relative">
                                        {unit.icon
                                            ? <div className="technologyIcon">
                                                <img alt="icon" src={unit.icon} />
                                            </div>
                                            : ''
                                        }
                                        <strong className="purple">
                                            <HighlightedText text={unit.name} highlight={hl} className="fs18" />
                                        </strong>
                                        <span className="grey light" style={{ marginLeft: '.5rem' }}>{unit.level ? unit.level : ''}</span>
                                        <ul>
                                            {
                                                unit.technologies.map((tech: any) => {
                                                    if (tech.name && tech.level && tech.content) {
                                                        return (
                                                            <li className="grey marginTop fs15" key={tech.id} >
                                                                <span>
                                                                    {tech.name
                                                                        + ' - '
                                                                        + tech.level
                                                                        + ': '}
                                                                </span>
                                                                <ReactMarkdown
                                                                    className="inlineBlock fs12"
                                                                    allowDangerousHtml={true}
                                                                    source={addMarkToMarkDown(tech.content, hl)}
                                                                />
                                                            </li>
                                                        );
                                                    } else if (tech.name && tech.content) {
                                                        return (
                                                            <li className="grey marginTop inlineChilds" key={tech.id} >
                                                                <span className="regular purple fs15">
                                                                    {tech.name}:
                                                                </span>
                                                                <ReactMarkdown
                                                                    source={addMarkToMarkDown(tech.content, hl)}
                                                                    allowDangerousHtml={true}
                                                                    className="fs12"
                                                                />
                                                            </li>
                                                        );
                                                    } else if (tech.name && tech.level) {
                                                        return (
                                                            <li className="grey marginTop" key={tech.id} >
                                                                <span className="regular purple">
                                                                    <HighlightedText highlight={hl} text={tech.name} className="fs15" />
                                                                </span>
                                                                : <HighlightedText highlight={hl} text={tech.level} className="fs12" />
                                                            </li>
                                                        );
                                                    } else {
                                                        return (
                                                            <li key={tech.id} className="grey marginTop">
                                                                <span className="purple">
                                                                    <HighlightedText highlight={hl} text={tech.name} className="fs12" />
                                                                </span>
                                                            </li>);
                                                    }
                                                })
                                            }
                                        </ul>
                                    </li>
                                </ul>
                            )
                        )
                    }
                    <Elm
                        src={SkillTimeLine.Elm.Elm.Components.Skills}
                        flags={{ style: skillsStyle, activeCategory: branch, lang: language }}
                        ports={setupSkillsPorts}
                    />
                </div>
                <div className="spacer" />
                <div className="section" id="hobbies" ref={hobbiesRef}>
                    <h1 className="purple regular">Loisirs</h1>
                    <div className="separator-h onlyMobile" />
                    <ul>
                        {
                            Dataset.hobbies.map((unit: any) => (
                                <li key={unit.id} className="marginTop purple mainList-item">
                                    <span className="fs15">{unit.name}</span>
                                    <ul>
                                        {
                                            unit.content.map((content: any) =>
                                                <li key={content.id} className="marginTop grey">
                                                    <ReactMarkdown
                                                        allowDangerousHtml={true}
                                                        source={addMarkToMarkDown(content.content, hl)}
                                                        className="fs12"
                                                    />
                                                </li>
                                            )
                                        }
                                    </ul>
                                </li>
                            ))
                        }
                    </ul>
                </div>
                <div className="spacer" />
                <div className="section" id="portfolio" ref={portfolioRef}>
                    <h1 className="purple regular">Portfolio</h1>
                    <Elm
                        src={Portfolio.Elm.Elm.Components.Portfolio}
                        flags={{ style: portfolioStyle, env, jwtToken, slug, activeCategory: branch, lang: language }}
                        ports={setupPortfolioPorts}
                    />
                </div>
                <footer>
                    <div>
                        <span>Built with :</span>
                    </div>
                    <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
                        <a href="http://spacemacs.org" style={{ marginRight: '1rem' }}>
                            <img
                                alt="Built with Spacemacs"
                                src="https://upload.wikimedia.org/wikipedia/commons/1/1c/Spacemacs_logo.svg"
                                style={{ height: '3.5rem' }}
                            />
                        </a>
                        <a href="https://github.com/doomemacs/doomemacs">
                            <img
                                alt="And built with Doom Emacs"
                                src={DOOM_ICON}
                                style={{ height: '3.5rem' }}
                            />
                        </a>
                    </div>
                    <div style={{ margin: '1rem' }}>
                        <span>Built On :</span>
                    </div>
                    <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
                        <img
                            alt="Built on macOS"
                            src="https://upload.wikimedia.org/wikipedia/commons/8/84/Apple_Computer_Logo_rainbow.svg"
                            style={{ height: '3.5rem', marginRight: '1rem' }}
                        />
                        <img
                            alt="And built on Manjaro"
                            src="https://upload.wikimedia.org/wikipedia/commons/a/a5/Manjaro_logo_text.png"
                            style={{ height: '3.5rem' }}
                        />
                    </div>
                    <div style={{ margin: '1rem' }}>
                        <span>Hosted On :</span>
                    </div>
                    <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
                        <img
                            alt="Backend hosted on DigitalOcean"
                            src="https://upload.wikimedia.org/wikipedia/commons/f/ff/DigitalOcean_logo.svg"
                            style={{ height: '3.5rem' }}
                        />
                    </div>
                </footer>
            </React.Fragment >
        );
    }
);
