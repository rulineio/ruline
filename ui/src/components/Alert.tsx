import clsx from 'clsx';

export interface AlertProps {
    message: string;
    type: 'success' | 'error' | 'warning' | 'info';
}

export const Alert: React.FC<AlertProps> = (props: AlertProps) => {
    const { message, type } = props;

    const alertClass = clsx('px-3 py-2 rounded-md', {
        'bg-green-100 border-green-400 text-green-700': type === 'success',
        'bg-red-100 border-red-400 text-red-700': type === 'error',
        'bg-yellow-100 border-yellow-400 text-yellow-700': type === 'warning',
        'bg-blue-100 border-blue-400 text-blue-700': type === 'info',
    });

    return (
        <div className={alertClass}>
            <span className="text-xs font-semibold">{message}</span>
        </div>
    );
};
