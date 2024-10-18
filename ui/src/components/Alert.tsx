import clsx from 'clsx';

export interface AlertProps {
    message: string;
    type: 'success' | 'error' | 'warning';
}

export const Alert: React.FC<AlertProps> = (props: AlertProps) => {
    const { message, type } = props;

    const alertClass = clsx(
        'px-3 py-2 ring-2 ring-inset rounded-md text-center',
        {
            'bg-success/10 ring-success/30 text-success': type === 'success',
            'bg-error/10 ring-error/30 text-error': type === 'error',
            'bg-warn/10 ring-warn/30 text-warn': type === 'warning',
        },
    );

    return (
        <div className={alertClass}>
            <span className="text-xs font-semibold">{message}</span>
        </div>
    );
};
