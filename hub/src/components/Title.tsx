import type { FC } from 'react';
import styles from '../styles/title.module.css';
import { IconContext } from 'react-icons';
import { SiDocsdotrs, SiGithub } from 'react-icons/si';

export const Title: FC = () => {
  return (
    <div className={styles.box}>
      <h1 className={styles.title}>scarlet-queen rustdoc</h1>
      <div className={styles.linkBox}>
        <IconContext.Provider value={{ color: '#000000', size: '60px' }}>
          <a
            href="https://github.com/pokemon-GA/scarlet-queen"
            target="_blank"
            rel="noopener noreferrer"
            className={styles.link}
          >
            <SiGithub />
            <p className={styles.icon}>GitHub</p>
          </a>
          <a
            href="https://pokemon-ga.github.io/scarlet-queen/"
            target="_blank"
            rel="noopener noreferrer"
            className={styles.link}
          >
            <SiDocsdotrs />
            <p className={styles.icon}>Documents</p>
          </a>
        </IconContext.Provider>
      </div>
    </div>
  );
};
