import { cleanup, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it } from 'vitest';
import { App } from './App';

afterEach(cleanup);

describe('App', () => {
  it('renders the OpenDesk desktop landing view', () => {
    render(<App />);

    expect(screen.getByRole('heading', { name: 'OpenDesk' })).toBeTruthy();
    expect(screen.getByText('Desktop client scaffold')).toBeTruthy();
  });
});
