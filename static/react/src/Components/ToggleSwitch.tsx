import { Location } from 'history';
import * as React from 'react';
import { connect, ConnectedProps } from 'react-redux';
import { Dispatch } from 'redux';
import { switchBranch } from 'src/actions/ui';
import { Branch, StoreState } from '../Types';

interface State {
    value: boolean;
}

interface SelfProps {
    location: Location;
}

const mapStateProps = ({ ui }: StoreState) => ({
    branch: ui.BRANCH
});
const mapDispatchProps = (dispatch: Dispatch) => ({
    onChange: (value: Branch) => dispatch(switchBranch(value)),
});

const connector = connect(mapStateProps, mapDispatchProps);

type PropsFromRedux = ConnectedProps<typeof connector>;

type Props = SelfProps & PropsFromRedux;

const routesToDisableSwitch = ['/hobbies', '/blog'];

export default connector(
    class ToggleSwitch extends React.Component<Props, State>{
        constructor(props: Props) {
            super(props);
            this.onChange = this.onChange.bind(this);
        }
        public onChange(e: React.FormEvent<HTMLInputElement>) {
            const checked = e.currentTarget.checked;
            this.props.onChange(checked ? Branch.WOOD : Branch.SOFTWARE);
        }
        public render() {
            const { location } = this.props;
            const checked = this.props.branch === Branch.WOOD;
            const shouldDisabledSwich = routesToDisableSwitch.indexOf(location.pathname) !== -1;
            return (
                <label
                    className="switch"
                    title={shouldDisabledSwich ? 'Cette fonctionnalité n\'est pas disponible sur cette page' : ''}
                >
                    <input type="checkbox" onChange={this.onChange} checked={checked} disabled={shouldDisabledSwich} />
                    <span className={['slider', 'round', shouldDisabledSwich ? 'disabled' : ''].join(' ')} />
                </label>
            );
        }
    }
);
