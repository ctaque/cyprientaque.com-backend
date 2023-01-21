import * as React from 'react';
import { Helmet } from 'react-helmet';
import { Transition } from 'react-transition-group';


interface IState {
    index: number;
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

class GenericNotFound extends React.Component<{}, IState>{

    public traductions: string[] = [
        'Page non trouvée',
        'Page not found',
        'পাওয়া যায় নি',
        'Չի գտնվել',
        'غير معثور عليه',
        'Nije pronađeno',
        'Nicht gefunden',
        '未找到',
        'Δεν βρέθηκε',
        'Extraviado'
    ];

    public interval: any;

    constructor(props: any) {
        super(props);
        this.state = {
            index: 0
        }
        this.setTheInterval = this.setTheInterval.bind(this);
    }

    public componentWillMount() {
        this.setTheInterval();
    }

    public componentWillUnmount() {
        clearInterval(this.interval);
    }

    public setTheInterval() {
        this.interval = setInterval(() => {
            const { index } = this.state;
            this.setState({
                index: index === this.traductions.length - 1 ? 0 : index + 1
            });
        }, 3000)
    }

    public render() {

        const text = this.traductions[this.state.index];

        return (
            <div className="notFound">
                <Helmet>
                    <title>404 Page non trouvée | Cyprien Taque</title>
                </Helmet>
                <div className="panel">
                    <Transition key={Math.random() * 1000} unmountOnExit={true} appear={true} exit={false} in={true} timeout={{ enter: 300, exit: 100 }} >
                        {(status: string) => (
                            <h1 className="purple thin" style={{ ...defaultStyle, ...transitionStyles[status] }}>{text}</h1>
                        )}
                    </Transition>
                </div>
            </div>
        )

    }
}

export default GenericNotFound;
