import Head from 'next/head'

import Layout from '../components/layout'
import Nav from '../components/nav'
import { FaBeer } from 'react-icons/fa';

import HighlightJS from '../components/highlightjs'
import Katex from '../components/katex'

import { fetchUtils } from 'react-admin'
const codeSample = `#[derive(Debug)]
pub enum State {
    Start,
    Transient,
    Closed,
}

impl From<&'a str> for State {
    fn from(s: &'a str) -> Self {
        match s {
            "start" => State::Start,
            "closed" => State::Closed,
            _ => unreachable!(),
        }
    }
}`

const mathExample = `
\\begin{aligned}
  E=m
\\end{aligned}
`;

const authProvider = {
  login: ({ clientId, clientSecret }) => {
    const request = new Request('https://github.com/login/oauth/authorize?client_id=0d12a9525a2fe2a02', {
      method: 'GET',
    });
    return fetch(request)
      .then(response => {
        if (response.status < 200 || response.status >= 300) {
          throw new Error(response.statusText)
        }
      });
  }
}

const fetchJson = (url, options = {}) => {
  if (!options.headers) {
    options.headers = new Headers({ Accept: 'application/vnd.github.v3+json' })
  }
  return fetchUtils.fetchJson(url, options)
}

  return (
    <Layout>
      <Head>
        <title>MonQ</title>
      </Head>
      <Nav />
      <div className="py-20">
        <h1 className="text-5xl text-center text-accent-1">
          MonQはクイズ・ベースの学習システムです.
        </h1>
        <div className="flex justify-center leading-none text-2xl">
          アイコンの表示:<span><FaBeer className="bg-orange-500" /></span>
        </div>
        <div>
          こんな感じでコードを表示できます.
        </div>
        <div>
          <HighlightJS codeString={codeSample} language="rust" />
        </div>
        <div>
          また数式も表示できます.
        </div>
        <div>
          <Katex source={mathExample} />
        </div>
      </div>
    </Layout>
  )
}