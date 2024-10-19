import * as RAvatar from '@radix-ui/react-avatar';
import * as R from 'remeda';
import cn from './utils/cn';

export interface AvatarProps {
    size?: 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10;
    className?: string;
    src?: string;
    name: string;
}
export function Avatar(props: AvatarProps) {
    const { src, name, size = 6, className } = props;

    const avatarClass = cn(
        {
            'size-1': size === 1,
            'size-2': size === 2,
            'size-3': size === 3,
            'size-4': size === 4,
            'size-5': size === 5,
            'size-6': size === 6,
            'size-7': size === 7,
            'size-8': size === 8,
            'size-9': size === 9,
            'size-10': size === 10,
        },
        className,
    );

    return (
        <RAvatar.Root className={avatarClass}>
            <RAvatar.Image className="rounded-full" src={src} alt={name} />
            <RAvatar.Fallback className="text-sm rounded-full bg-teal-9 text-white w-full h-full flex items-center justify-center">
                {R.first(R.split(name, ''))}
            </RAvatar.Fallback>
        </RAvatar.Root>
    );
}
