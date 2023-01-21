import * as moment from 'moment';
import * as React from 'react';
import { connect, ConnectedProps } from 'react-redux';
import ReactSVG from 'react-svg';
import { Dispatch } from 'redux';
import * as uiActions from '../actions/ui';
import { T } from '../internationalization/Trans';
import bitbucketSVG from '../svg/light_purple/bitbucket.svg';
import emailSVG from '../svg/light_purple/gmail.svg';
import pdfSVG from '../svg/light_purple/pdf.svg';
import placeSVG from '../svg/light_purple/placeholder.svg';
import { Branch, StoreState } from '../Types';
import DesktopNav from './DesktopNav';

const mapStateProps = (state: StoreState) => ({
    branch: state.ui.BRANCH,
});

const mapDispatchProps = (dispatch: Dispatch) => ({
    changeBranch: (branch: Branch) => dispatch(uiActions.switchBranch(branch))
});

const svgStyle = { width: '18px', height: '18px' };

const connector = connect(mapStateProps, mapDispatchProps);

type Props = ConnectedProps<typeof connector>;

export default connector(
    ({ branch, changeBranch }: Props) => {

        return (
            <header id="main-header">
                <div className="PageWrap header">
                    <div className="rowWrapper">
                        <div className="headline">
                            <h1 className="regular">{T.translate('header.mainTitle')}</h1>
                            <h3 className="regular purple-light">{T.translate('header.subTitle')}</h3>
                        </div>
                        <div className="contactHeader onlyDesktop">
                            <ul>
                                <li className="white regular">Cyprien Taque</li>
                                <li>
                                    <div className="svgInline">
                                        <ReactSVG path={emailSVG}
                                            svgStyle={Object.assign({ paddingTop: '3px' }, svgStyle)} />
                                    </div>
                                    <a href="mailto:cyprien.taque@gmail.com">cyprien.taque@gmail.com</a>
                                </li>
                                <li>
                                    <div className="svgInline">
                                        <ReactSVG path={placeSVG}
                                            svgStyle={Object.assign({ paddingTop: '0px' }, svgStyle)} />
                                    </div>
                            Nantes 44300
                            </li>
                                <li>
                                    <div className="svgInline">
                                        <ReactSVG path={bitbucketSVG}
                                            svgStyle={Object.assign({ paddingTop: '2px' }, svgStyle)} />
                                    </div>
                                    <a href="https://bitbucket.com/syprex" target="_blank">bitbucket</a></li>
                                <li>{moment().diff(moment('1990-05-05'), 'years')} {T.translate('home.age')}</li>
                                <li className="relative">
                                    <div className="svgInline">
                                        <ReactSVG path={pdfSVG}
                                            svgStyle={Object.assign({ paddingTop: '0px' }, svgStyle)} />
                                    </div>
                                    <a className="withGreenDot" target="_blank" href="https://s3-eu-west-1.amazonaws.com/ctaque.logos/cv_Cyprien_Taque.pdf">{T.translate('home.resume')}</a>
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>
                <DesktopNav id="desktop-nav" classNames={['desktop']} />
                <DesktopNav id="desktop-nav-fixed" classNames={['desktop', 'onlyDesktop', 'fixed', 'hidden']} />
            </header>

        );
    });
