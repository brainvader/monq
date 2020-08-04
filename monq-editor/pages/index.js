import Head from 'next/head'

import Layout from '../components/layout'
import Nav from '../components/nav'
import { FaBeer } from 'react-icons/fa';

import HighlightJS from '../components/highlightjs'

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

export default function Editor() {
  return (
    <Layout>
      <Head>
        <title>MonQ Editor</title>
      </Head>
      <Nav />
      <div className="py-20">
        <h1 className="text-5xl text-center text-accent-1">
          Next.js + Tailwind CSS
        </h1>
        <div className="flex justify-center leading-none text-2xl">
          これがアイコンです<span><FaBeer className="bg-orange-500" /></span>
        </div>
        <div>
          <HighlightJS codeString={codeSample} language="rust" />
        </div>
      </div>
    </Layout>
  )
}