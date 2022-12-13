// Hey Emacs, this is -*- coding: utf-8 -*-

import path from 'path';

import { config } from 'dotenv';
import { expand } from 'dotenv-expand';

import { packageRoot, repositoryRoot } from '~/roots';

const DOTENV_FILENAME =
  process.env.DOTENV_FILENAME === undefined
    ? '.env'
    : process.env.DOTENV_FILENAME;

expand(
  config({
    path: path.join(packageRoot, DOTENV_FILENAME),
  }),
);

expand(
  config({
    path: path.join(repositoryRoot, DOTENV_FILENAME),
  }),
);
