import { defineConfig } from 'astro/config';
import tailwindcss from '@tailwindcss/vite';
import react from '@astrojs/react';

const isGithubActions = process.env.GITHUB_ACTIONS || false;
const repoName = process.env.GITHUB_REPOSITORY ? process.env.GITHUB_REPOSITORY.split('/')[1] : '';

export default defineConfig({
  site: isGithubActions ? `https://${process.env.GITHUB_REPOSITORY_OWNER}.github.io` : 'http://localhost:4321',
  base: isGithubActions ? `/${repoName}/` : '/',
  vite: {
    plugins: [tailwindcss()],
  },
  integrations: [react()],
});
