export interface AlertProps {
    message: string;
    type: 'success' | 'error' | 'warning' | 'info';
}

export const Alert: React.FC<AlertProps> = (props: AlertProps) => {
    const { message, type } = props;

    const alertClasses = {
        success: 'bg-green-100 border-green-400 text-green-700',
        error: 'bg-red-100 border-red-400 text-red-700',
        warning: 'bg-yellow-100 border-yellow-400 text-yellow-700',
        info: 'bg-blue-100 border-blue-400 text-blue-700',
    };

    return (
        <div className={`px-3 py-2 rounded-md ${alertClasses[type]}`}>
            <span className="text-xs font-semibold">{message}</span>
        </div>
    );
};
