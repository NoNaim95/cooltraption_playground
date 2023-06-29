use crate::RuntimeConfigurationBuilder;

pub mod common_configurators;

pub trait Configurator: ConfiguratorOnce {
    fn configure(&self, runtime_config: &mut RuntimeConfigurationBuilder);
}

pub trait ConfiguratorOnce {
    fn configure_once(self: Box<Self>, runtime_config: &mut RuntimeConfigurationBuilder);
}

#[derive(Default)]
pub struct ConfiguratorPipeline<'a> {
    configurators: Vec<Box<dyn Configurator + 'a>>,
}

impl<'a> ConfiguratorPipeline<'a> {
    pub fn add_configurator(&mut self, configurator: impl Configurator + 'a) -> &mut Self {
        self.configurators.push(Box::new(configurator));
        self
    }
}

impl<'a> Configurator for ConfiguratorPipeline<'a> {
    fn configure(&self, runtime_config: &mut RuntimeConfigurationBuilder) {
        for configurator in &self.configurators {
            configurator.configure(runtime_config);
        }
    }
}

impl<'c> ConfiguratorOnce for ConfiguratorPipeline<'c> {
    fn configure_once(self: Box<Self>, runtime_config: &mut RuntimeConfigurationBuilder) {
        self.configure(runtime_config)
    }
}

#[derive(Default)]
pub struct ConfiguratorOncePipeline<'a> {
    configurators: Vec<Box<dyn ConfiguratorOnce + 'a>>,
}

impl<'a> ConfiguratorOncePipeline<'a> {
    pub fn add_configurator_once(
        &mut self,
        configurator_once: impl ConfiguratorOnce + 'a,
    ) -> &mut Self {
        self.configurators.push(Box::new(configurator_once));
        self
    }
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl<'a> ConfiguratorOnce for ConfiguratorOncePipeline<'a> {
    fn configure_once(self: Box<Self>, runtime_config: &mut RuntimeConfigurationBuilder) {
        for configurator in self.configurators {
            configurator.configure_once(runtime_config);
        }
    }
}

impl<F> Configurator for F
where
    F: Fn(&mut RuntimeConfigurationBuilder),
{
    fn configure(&self, runtime_config: &mut RuntimeConfigurationBuilder) {
        self(runtime_config)
    }
}

impl<F> ConfiguratorOnce for F
where
    F: FnOnce(&mut RuntimeConfigurationBuilder),
{
    fn configure_once(self: Box<Self>, runtime_config: &mut RuntimeConfigurationBuilder) {
        self(runtime_config)
    }
}
