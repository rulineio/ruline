import * as Label from '@radix-ui/react-label';
import type { FieldValues, Path, UseFormRegister } from 'react-hook-form';
import cn from './utils/cn';

export interface InputProps<T extends FieldValues>
    extends React.InputHTMLAttributes<HTMLInputElement> {
    label: string;
    name: Path<T>;
    register: UseFormRegister<T>;
    error?: string;
    optional?: boolean;
}

export function Input<T extends FieldValues>(props: InputProps<T>) {
    const { label, name, register, error, className, optional, ...rest } =
        props;

    const inputClass = cn(
        'w-full px-3 py-2 rounded-md shadow-sm',
        'outline outline-1 focus:ring-2',
        'sm:text-sm',
        {
            'bg-gray-2 outline-gray-6 focus:ring-teal-8 text-gray-11': !error,
            'bg-red-2 outline-red-6 focus:ring-red-8 text-red-11': !!error,
        },
        className,
    );
    const errorClass = cn('text-red-11 text-xs', {
        hidden: !error,
    });

    return (
        <div className="space-y-2">
            <Label.Root
                htmlFor={name}
                className="text-xs text-white text-opacity-60"
            >
                {label}
                {optional && (
                    <span className="text-xs opacity-65"> (optional)</span>
                )}
            </Label.Root>
            <input
                id={name}
                className={inputClass}
                {...register(name, { required: !optional })}
                {...rest}
            />
            <span className={errorClass}>{error ?? 'Some Error'}</span>
        </div>
    );
}
