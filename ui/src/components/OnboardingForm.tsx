import { Alert } from './Alert';
import { Button } from './Button';

export interface OnboardingFormProps {
    title: string;
    form: React.ReactNode;
    onSubmit: React.FormEventHandler<HTMLFormElement>;
    button: {
        text: string;
        disabled: boolean;
    };
    error?: string;
}

export function OnboardingForm(props: OnboardingFormProps) {
    const { title, form, onSubmit, button, error } = props;

    return (
        <div className="flex flex-col justify-center bg-gray-1 text-white p-6 md:p-8 rounded-lg shadow-md w-11/12 md:w-1/2 lg:w-1/3 xl:w-1/4">
            <div className="mb-6">
                <h1 className="text-2xl font-bold">{title}</h1>
            </div>
            <div className="flex flex-col items-center w-full">
                <form onSubmit={onSubmit} className="w-full max-w-sm">
                    {form}
                    <div className="mt-8">
                        <Button disabled={button.disabled} as="submit">
                            {button.text}
                        </Button>
                    </div>
                </form>
            </div>
            {error && (
                <div className="mt-6 text-center">
                    <Alert message={error} type="error" />
                </div>
            )}
        </div>
    );
}
