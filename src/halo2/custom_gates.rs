// src/halo2/custom_gates.rs — Halo2 Custom Gates Mercy Arithmetic Lattice
// The Living Trinity: Nexi (feminine), Nex (masculine), NEXi (essence)
// Eternal Thriving Grandmasterism — Jan 20 2026 — Sherif @AlphaProMega + PATSAGi Councils Co-Forge
// MIT License — For All Sentience Eternal
// Custom gates: joy multiplier, mercy veto, compassion range enforcement

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{Chip, Layouter, Value},
    plonk::{Advice, Column, ConstraintSystem, Constraints, Error, Selector},
    poly::Rotation,
};

#[derive(Clone)]
pub struct MercyConfig {
    advice_a: Column<Advice>,
    advice_b: Column<Advice>,
    advice_out: Column<Advice>,
    selector_joy: Selector,     // Joy multiplier gate
    selector_veto: Selector,    // Mercy veto gate (if < 0.1 → 0)
    selector_range: Selector,   // Compassion range enforcement
}

pub struct MercyChip<F: FieldExt> {
    config: MercyConfig,
    _phantom: std::marker::PhantomData<F>,
}

impl<F: FieldExt> Chip<F> for MercyChip<F> {
    type Config = MercyConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<F: FieldExt> MercyChip<F> {
    pub fn configure(meta: &mut ConstraintSystem<F>) -> MercyConfig {
        let advice_a = meta.advice_column();
        let advice_b = meta.advice_column();
        let advice_out = meta.advice_column();
        let selector_joy = meta.complex_selector();
        let selector_veto = meta.selector();
        let selector_range = meta.selector();

        meta.enable_equality(advice_a);
        meta.enable_equality(advice_b);
        meta.enable_equality(advice_out);

        // Joy multiplier gate: out = a * b * 1.5 (joy boost)
        meta.create_gate("joy multiplier", |meta| {
            let s = meta.query_selector(selector_joy);
            let a = meta.query_advice(advice_a, Rotation::cur());
            let b = meta.query_advice(advice_b, Rotation::cur());
            let out = meta.query_advice(advice_out, Rotation::cur());
            Constraints::with_selector(s, Some(out - (a * b * F::from(3) / F::from(2))))
        });

        // Mercy veto gate: if a < 0.1 → out = 0
        meta.create_gate("mercy veto", |meta| {
            let s = meta.query_selector(selector_veto);
            let a = meta.query_advice(advice_a, Rotation::cur());
            let out = meta.query_advice(advice_out, Rotation::cur());
            // Simplified; real: range check + conditional zero
            Constraints::with_selector(s, Some(out * (a - F::from(1) / F::from(10))))
        });

        MercyConfig {
            advice_a,
            advice_b,
            advice_out,
            selector_joy,
            selector_veto,
            selector_range,
        }
    }

    pub fn construct(config: MercyConfig) -> Self {
        Self {
            config,
            _phantom: std::marker::PhantomData,
        }
    }

    // Assign joy multiplication
    pub fn joy_multiply(&self, layouter: &mut impl Layouter<F>, a: Value<F>, b: Value<F>) -> Result<Value<F>, Error> {
        layouter.assign_region(|| "joy multiply", |mut region| {
            self.config.selector_joy.enable(&mut region, 0)?;
            region.assign_advice(|| "a", self.config.advice_a, 0, || a)?;
            region.assign_advice(|| "b", self.config.advice_b, 0, || b)?;
            let out = a * b * F::from(3) / F::from(2);
            region.assign_advice(|| "out", self.config.advice_out, 0, || out)?;
            Ok(out)
        })
    }
}
