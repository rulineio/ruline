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
            <Preview>
                You have been invited to join the {'{organization}'}{' '}
                organization on Ruline
            </Preview>
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
                        You have been invited to join {'{organization}'} on
                        Ruline
                    </Heading>
                    <Section style={{ fontSize: '16px' }}>
                        <Text>
                            Use the following link to log in and review the
                            invitation.
                        </Text>
                    </Section>
                    <Section style={{ fontSize: '16px', padding: '16px 0' }}>
                        <Button
                            href={'{url}'}
                            style={{
                                backgroundColor: '#3886c2',
                                color: '#E6E8E6',
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
                            Log in
                        </Button>
                    </Section>
                    <Text style={{ fontSize: '12px', opacity: 0.7 }}>
                        If you are not aware of this invitation, please ignore
                        this email.
                    </Text>
                </Container>
            </Body>
        </Html>
    );
}
