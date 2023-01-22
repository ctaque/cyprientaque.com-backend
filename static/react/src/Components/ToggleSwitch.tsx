import * as React from 'react';
import { connect, ConnectedProps } from 'react-redux';
import { Dispatch } from 'redux';
import { switchBranch } from '../actions/ui';
import { Branch, StoreState } from '../Types';

interface State {
    value: boolean;
}

const mapStateProps = ({ ui }: StoreState) => ({
    branch: ui.BRANCH
});
const mapDispatchProps = (dispatch: Dispatch) => ({
    onChange: (value: Branch) => dispatch(switchBranch(value)),
});

const connector = connect(mapStateProps, mapDispatchProps);

type PropsFromRedux = ConnectedProps<typeof connector>;

type Props = PropsFromRedux;

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
            const checked = this.props.branch === Branch.WOOD;
            return (
                <label
                    className="switch"
                >
                    <input type="checkbox" onChange={this.onChange} checked={checked} />
                    <span className={['slider', 'round'].join(' ')} />
                </label>
            );
        }
    }
);
