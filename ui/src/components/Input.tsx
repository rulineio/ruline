import clsx from 'clsx';
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

    const inputClass = clsx(
        'mt-2 block w-full px-3 py-2 rounded-md shadow-sm focus:outline-none',
        'sm:text-sm bg-background text-primary-text autofill:bg-transparent',
        className,
        {
            'ring-red-200 ring-2': !!error,
            'border border-primary focus:ring-2 focus:ring-accent': !error,
        },
    );
    const labelClass = clsx('text-xs', {
        'text-error': !!error,
    });
    const errorClass = clsx('mt-1 text-xs text-error leading-3', {
        hidden: !error,
    });

    return (
        <div className="my-2">
            <label htmlFor={name} className={labelClass}>
                {label}
                {rest.optional && (
                    <span className="text-xs opacity-65"> (optional)</span>
                )}
            </label>
            <input
                id={name}
                className={inputClass}
                {...register(name, { required: !rest.optional })}
                {...rest}
            />
            <span className={errorClass}>{error}</span>
        </div>
    );
}
