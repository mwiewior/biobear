use pyo3::prelude::*;

mod bam_reader;
mod batch;
mod exon_reader;
mod genbank_reader;
mod to_arrow;
mod vcf_reader;

#[pymodule]
fn biobear(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<exon_reader::ExonReader>()?;

    m.add_class::<bam_reader::BamReader>()?;
    m.add_class::<bam_reader::BamIndexedReader>()?;

    m.add_class::<vcf_reader::VCFReader>()?;
    m.add_class::<vcf_reader::VCFIndexedReader>()?;

    m.add_class::<genbank_reader::GenbankReader>()?;

    Ok(())
}
