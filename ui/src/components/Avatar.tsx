import clsx from 'clsx';
import * as R from 'remeda';

export type AvatarProps = {
    size?: 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10;
    className?: string;
} & (
    | {
          src?: string;
          name?: never;
      }
    | {
          src?: never;
          name: string;
      }
);

export function Avatar(props: AvatarProps) {
    const { src, name, size = 6, className } = props;

    const avatarClass = clsx('rounded-full', className, {
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
        'flex items-center justify-center bg-accent text-accent-text': !src,
    });

    if (src) {
        return <img src={src} alt={name || 'avatar'} className={avatarClass} />;
    }

    if (name) {
        return (
            <div className={avatarClass}>
                <span>{R.capitalize(name[0])}</span>
            </div>
        );
    }
}
