import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { vi, describe, it, expect, beforeEach } from 'vitest';
import LoginScreen from '../LoginScreen';
import { api } from '../../services/api';

// Mock the API service
vi.mock('../../services/api', () => ({
    api: {
        listProfiles: vi.fn(),
        unlockIdentity: vi.fn(),
        selectProfile: vi.fn(),
        getIdentityStatus: vi.fn(),
        post: vi.fn(),
    },
}));

describe('LoginScreen', () => {
    const mockOnUnlock = vi.fn();
    const mockStatus = {
        locked: true,
        has_identity: true,
        username: 'Ghost_1234',
        did: 'did:ghost:abc123',
        setup_complete: true,
    };

    beforeEach(() => {
        vi.clearAllMocks();
    });

    it('renders progress to unlock mode by default when identity is present', () => {
        render(<LoginScreen onUnlock={mockOnUnlock} status={mockStatus} />);
        expect(screen.getByPlaceholderText(/ENTER PASSPHRASE/i)).toBeInTheDocument();
        expect(screen.getByText('Ghost_1234')).toBeInTheDocument();
    });

    it('calls unlockIdentity and onUnlock on successful login', async () => {
        (api.unlockIdentity as any).mockResolvedValue(true);
        (api.getIdentityStatus as any).mockResolvedValue({ setup_complete: true });

        render(<LoginScreen onUnlock={mockOnUnlock} status={mockStatus} />);

        const passwordInput = screen.getByPlaceholderText(/ENTER PASSPHRASE/i);
        const submitButton = screen.getByText(/UNLOCK PROFILE/i);

        fireEvent.change(passwordInput, { target: { value: 'password123' } });
        fireEvent.click(submitButton);

        await waitFor(() => {
            expect(api.unlockIdentity).toHaveBeenCalledWith('password123');
            expect(mockOnUnlock).toHaveBeenCalled();
        });
    });

    it('shows error message on failed login', async () => {
        (api.unlockIdentity as any).mockResolvedValue(false);

        render(<LoginScreen onUnlock={mockOnUnlock} status={mockStatus} />);

        const passwordInput = screen.getByPlaceholderText(/ENTER PASSPHRASE/i);
        const submitButton = screen.getByText(/UNLOCK PROFILE/i);

        fireEvent.change(passwordInput, { target: { value: 'wrong_password' } });
        fireEvent.click(submitButton);

        await waitFor(() => {
            expect(screen.getByText(/ACCESS DENIED: Incorrect password/i)).toBeInTheDocument();
        });
    });

    it('switches to create profile mode when button is clicked', async () => {
        (api.listProfiles as any).mockResolvedValue({ profiles: [] });

        render(<LoginScreen onUnlock={mockOnUnlock} status={mockStatus} />);

        const switchButton = screen.getByText(/SWITCH PROFILE/i);
        fireEvent.click(switchButton);

        const createButton = await screen.findByText(/CREATE NEW PROFILE/i);
        fireEvent.click(createButton);

        expect(screen.getByPlaceholderText(/e.g. Shadow_Walker/i)).toBeInTheDocument();
    });
});
