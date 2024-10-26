import cn from './utils/cn';

export interface AlertProps {
    message: string;
    type: 'success' | 'error' | 'warning';
}

export const Alert: React.FC<AlertProps> = (props: AlertProps) => {
    const { message, type } = props;

    const alertClass = cn(
        'px-3 py-2 ring-2 ring-inset rounded-md text-center',
        {
            'bg-green-3 ring-green-7 text-green-11': type === 'success',
            'bg-red-3 ring-red-7 text-red-11': type === 'error',
            'bg-amber-3 ring-amber-7 text-amber-11': type === 'warning',
        },
    );

    return (
        <div className={alertClass}>
            <span className="text-xs font-semibold">{message}</span>
        </div>
    );
};
