import {
    Body,
    Button,
    Container,
    Head,
    Heading,
    Html,
    Preview,
    Section,
    Text,
} from '@react-email/components';

export default function Email() {
    return (
        <Html lang="en" dir="ltr">
            <Head />
            <Preview>Your link for Ruline</Preview>
            <Body
                style={{
                    backgroundColor: '#ffffff',
                    fontFamily: "'Roboto', sans-serif",
                }}
            >
                <Container
                    style={{
                        padding: '16px 0 48px',
                        maxWidth: '600px',
                        margin: '0 auto',
                    }}
                >
                    <Heading style={{ fontSize: '24px' }}>
                        Your link for Ruline
                    </Heading>
                    <Section style={{ fontSize: '16px', padding: '16px 0' }}>
                        <Button
                            href={'{url}'}
                            style={{
                                backgroundColor: '#0070f3',
                                color: '#ffffff',
                                padding: '8px 16px',
                                textDecoration: 'none',
                                borderRadius: '4px',
                                fontWeight: 'bold',
                                display: 'inline-block',
                                textAlign: 'center',
                                cursor: 'pointer',
                                border: 'none',
                            }}
                        >
                            Login
                        </Button>
                    </Section>
                    <Text style={{ fontSize: '12px', opacity: 0.7 }}>
                        The link is valid for 5 minutes. If you didn't request
                        this link, you can safely ignore this email.
                    </Text>
                </Container>
            </Body>
        </Html>
    );
}
