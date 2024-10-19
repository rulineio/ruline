import * as RSelect from '@radix-ui/react-select';
import * as Label from '@radix-ui/react-label';
import { Icon } from './Icon';
import cn from './utils/cn';

export interface SelectProps {
    value: string;
    placeholder?: string;
    onChange: (value: string) => void;
    options: { value: string; label: string }[];
    label: string;
    className?: string;
}

export function Select(props: SelectProps) {
    const { value, placeholder, onChange, options, label, className } = props;

    const triggerClass = cn(
        'inline-flex items-center justify-center space-x-2 px-3 py-2',
        'sm:text-sm font-medium',
        'rounded-md shadow-sm outline outline-1 focus:ring-2',
        'bg-gray-2 outline-gray-6 focus:ring-teal-8 text-gray-11',
    );
    const contentClass = cn(
        'overflow-hidden rounded-md',
        'bg-gray-2 text-white',
    );
    const viewportClass = cn('p-1');
    const itemClass = cn(
        'flex items-center justify-between space-x-2 py-2 px-3',
        'text-sm rounded-md cursor-default',
        'data-[highlighted]:bg-teal-5',
        'hover:bg-teal-4 focus:bg-teal-4 focus:outline-none',
    );

    return (
        <div className="space-y-2">
            <Label.Root
                htmlFor={label}
                className="text-xs text-white text-opacity-60"
            >
                {label}
            </Label.Root>
            <RSelect.Root value={value} onValueChange={onChange}>
                <RSelect.Trigger id={label} className={triggerClass}>
                    <RSelect.Value placeholder={placeholder} />
                    <RSelect.Icon>
                        <Icon size={4} icon="chevron-down" />
                    </RSelect.Icon>
                </RSelect.Trigger>

                <RSelect.Portal>
                    <RSelect.Content className={contentClass}>
                        <RSelect.Viewport className={viewportClass}>
                            {options.map((option) => (
                                <RSelect.Item
                                    key={option.value}
                                    value={option.value}
                                    className={itemClass}
                                >
                                    <RSelect.ItemText>
                                        {option.label}
                                    </RSelect.ItemText>
                                </RSelect.Item>
                            ))}
                        </RSelect.Viewport>
                    </RSelect.Content>
                </RSelect.Portal>
            </RSelect.Root>
        </div>
    );
}
