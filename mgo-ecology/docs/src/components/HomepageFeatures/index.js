import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

const FeatureList = [
  {
    title: 'Built for Web3',
    Svg: require('@site/static/img/undraw_docusaurus_mountain.svg').default,
    description: (
      <>
        Mango aims to be the most accessible smart contract platform, empowering developers to create great user experiences in web3. To usher in the next billion users, Mango empowers developers with various tools to take advantage of the power of the Mango blockchain. The Mango Development Kit (SDK) will enable developers to build without boundaries.
      </>
    ),
  },
  {
    title: 'Scalability',
    Svg: require('@site/static/img/undraw_docusaurus_tree.svg').default,
    description: (
      <>
        Mango scales horizontally to meet the demands of applications. Network capacity grows in proportion to the increase in Mango validators' processing power by adding workers, resulting in low gas fees even during high network traffic. This scalability characteristic is in sharp contrast to other blockchains with rigid bottlenecks.
      </>
    ),
  },
  {
    title: 'On-chain assets',
    Svg: require('@site/static/img/undraw_docusaurus_react.svg').default,
    description: (
      <>
        Rich on-chain assets enable new applications and economies based on utility without relying solely on artificial scarcity. Developers can implement dynamic NFTs that you can upgrade, bundle, and group in an application-specific manner, such as changes in avatars and customizable items based on gameplay. This capability delivers stronger in-game economies as NFT behavior gets fully reflected on-chain, making NFTs more valuable and delivering more engaging feedback loops.
      </>
    ),
  },
];

function Feature({Svg, title, description}) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
