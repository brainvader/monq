import Nav from '../components/nav'
import { FaBeer } from 'react-icons/fa';

export default function Editor() {
  return (
    <div>
      <Nav />
      <div className="py-20">
        <h1 className="text-5xl text-center text-accent-1">
          Next.js + Tailwind CSS
        </h1>
        <div className="flex justify-center leading-none text-2xl">これがアイコンです<span><FaBeer className="bg-orange-500" /></span></div>
      </div>
    </div >
  )
}