import type { FieldValues, Path, UseFormRegister } from 'react-hook-form';

export interface InputProps<T extends FieldValues>
    extends React.InputHTMLAttributes<HTMLInputElement> {
    label: string;
    name: Path<T>;
    register: UseFormRegister<T>;
    error?: string;
    optional?: boolean;
}

export function Input<T extends FieldValues>(props: InputProps<T>) {
    const { label, name, register, error, className, ...rest } = props;

    const classes = {
        error: 'ring-red-200 ring-2',
        normal: 'border border-gray-300 focus:ring-2 focus:ring-blue-500',
    };

    const labelClasses = {
        error: 'text-red-500',
        normal: '',
    };

    return (
        <div className="my-2">
            <label
                htmlFor={name}
                className={`text-sm ${labelClasses[error ? 'error' : 'normal']}`}
            >
                {label}
                {rest.optional && (
                    <span className="text-xs opacity-65"> (optional)</span>
                )}
            </label>
            <input
                id={name}
                className={`mt-2 block w-full px-3 py-2 rounded-md shadow-sm focus:outline-none sm:text-sm ${
                    error ? classes.error : classes.normal
                } ${className || ''}`}
                {...register(name, { required: !rest.optional })}
                {...rest}
            />
            {error && (
                <span className="mt-1 text-xs text-red-500 leading-3">
                    {error}
                </span>
            )}
        </div>
    );
}
