use super::enums::ModelType;
use super::structs_core::{
    AssetAdministrationShell, AssetInformation, ConceptDescription, Environment, Submodel,
};
use super::structs_elements::{
    AdministrativeInformation, AnnotatedRelationshipElement, BasicEventElement, Blob, Capability,
    EmbeddedDataSpecification, Entity, File, MultiLanguageProperty, Operation, Property, Range,
    ReferenceElement, RelationshipElement, SubmodelElementCollection, SubmodelElementList,
};
use super::structs_lang::{
    DataSpecificationIec61360, LangStringDefinitionTypeIec61360, LangStringNameType,
    LangStringPreferredNameTypeIec61360, LangStringShortNameTypeIec61360, LangStringTextType,
};
use super::structs_support::{
    EventPayload, Extension, Key, LevelType, OperationVariable, Qualifier, Reference,
    SpecificAssetId, ValueList, ValueReferencePair, Resource,
};

/// Wraps all 38 concrete AAS types.
#[derive(Debug, Clone)]
pub enum Class {
    Extension(Extension),
    AdministrativeInformation(AdministrativeInformation),
    Qualifier(Qualifier),
    AssetAdministrationShell(AssetAdministrationShell),
    AssetInformation(AssetInformation),
    Resource(Resource),
    SpecificAssetId(SpecificAssetId),
    Submodel(Submodel),
    RelationshipElement(RelationshipElement),
    SubmodelElementList(SubmodelElementList),
    SubmodelElementCollection(SubmodelElementCollection),
    Property(Property),
    MultiLanguageProperty(MultiLanguageProperty),
    Range(Range),
    ReferenceElement(ReferenceElement),
    Blob(Blob),
    File(File),
    AnnotatedRelationshipElement(AnnotatedRelationshipElement),
    Entity(Entity),
    EventPayload(EventPayload),
    BasicEventElement(BasicEventElement),
    Operation(Operation),
    OperationVariable(OperationVariable),
    Capability(Capability),
    ConceptDescription(ConceptDescription),
    Reference(Reference),
    Key(Key),
    LangStringNameType(LangStringNameType),
    LangStringTextType(LangStringTextType),
    Environment(Environment),
    EmbeddedDataSpecification(EmbeddedDataSpecification),
    LevelType(LevelType),
    ValueReferencePair(ValueReferencePair),
    ValueList(ValueList),
    LangStringPreferredNameTypeIec61360(LangStringPreferredNameTypeIec61360),
    LangStringShortNameTypeIec61360(LangStringShortNameTypeIec61360),
    LangStringDefinitionTypeIec61360(LangStringDefinitionTypeIec61360),
    DataSpecificationIec61360(DataSpecificationIec61360),
}

// From implementations for all 38 types
impl From<Extension> for Class {
    fn from(v: Extension) -> Self {
        Class::Extension(v)
    }
}
impl From<AdministrativeInformation> for Class {
    fn from(v: AdministrativeInformation) -> Self {
        Class::AdministrativeInformation(v)
    }
}
impl From<Qualifier> for Class {
    fn from(v: Qualifier) -> Self {
        Class::Qualifier(v)
    }
}
impl From<AssetAdministrationShell> for Class {
    fn from(v: AssetAdministrationShell) -> Self {
        Class::AssetAdministrationShell(v)
    }
}
impl From<AssetInformation> for Class {
    fn from(v: AssetInformation) -> Self {
        Class::AssetInformation(v)
    }
}
impl From<Resource> for Class {
    fn from(v: Resource) -> Self {
        Class::Resource(v)
    }
}
impl From<SpecificAssetId> for Class {
    fn from(v: SpecificAssetId) -> Self {
        Class::SpecificAssetId(v)
    }
}
impl From<Submodel> for Class {
    fn from(v: Submodel) -> Self {
        Class::Submodel(v)
    }
}
impl From<RelationshipElement> for Class {
    fn from(v: RelationshipElement) -> Self {
        Class::RelationshipElement(v)
    }
}
impl From<SubmodelElementList> for Class {
    fn from(v: SubmodelElementList) -> Self {
        Class::SubmodelElementList(v)
    }
}
impl From<SubmodelElementCollection> for Class {
    fn from(v: SubmodelElementCollection) -> Self {
        Class::SubmodelElementCollection(v)
    }
}
impl From<Property> for Class {
    fn from(v: Property) -> Self {
        Class::Property(v)
    }
}
impl From<MultiLanguageProperty> for Class {
    fn from(v: MultiLanguageProperty) -> Self {
        Class::MultiLanguageProperty(v)
    }
}
impl From<Range> for Class {
    fn from(v: Range) -> Self {
        Class::Range(v)
    }
}
impl From<ReferenceElement> for Class {
    fn from(v: ReferenceElement) -> Self {
        Class::ReferenceElement(v)
    }
}
impl From<Blob> for Class {
    fn from(v: Blob) -> Self {
        Class::Blob(v)
    }
}
impl From<File> for Class {
    fn from(v: File) -> Self {
        Class::File(v)
    }
}
impl From<AnnotatedRelationshipElement> for Class {
    fn from(v: AnnotatedRelationshipElement) -> Self {
        Class::AnnotatedRelationshipElement(v)
    }
}
impl From<Entity> for Class {
    fn from(v: Entity) -> Self {
        Class::Entity(v)
    }
}
impl From<EventPayload> for Class {
    fn from(v: EventPayload) -> Self {
        Class::EventPayload(v)
    }
}
impl From<BasicEventElement> for Class {
    fn from(v: BasicEventElement) -> Self {
        Class::BasicEventElement(v)
    }
}
impl From<Operation> for Class {
    fn from(v: Operation) -> Self {
        Class::Operation(v)
    }
}
impl From<OperationVariable> for Class {
    fn from(v: OperationVariable) -> Self {
        Class::OperationVariable(v)
    }
}
impl From<Capability> for Class {
    fn from(v: Capability) -> Self {
        Class::Capability(v)
    }
}
impl From<ConceptDescription> for Class {
    fn from(v: ConceptDescription) -> Self {
        Class::ConceptDescription(v)
    }
}
impl From<Reference> for Class {
    fn from(v: Reference) -> Self {
        Class::Reference(v)
    }
}
impl From<Key> for Class {
    fn from(v: Key) -> Self {
        Class::Key(v)
    }
}
impl From<LangStringNameType> for Class {
    fn from(v: LangStringNameType) -> Self {
        Class::LangStringNameType(v)
    }
}
impl From<LangStringTextType> for Class {
    fn from(v: LangStringTextType) -> Self {
        Class::LangStringTextType(v)
    }
}
impl From<Environment> for Class {
    fn from(v: Environment) -> Self {
        Class::Environment(v)
    }
}
impl From<EmbeddedDataSpecification> for Class {
    fn from(v: EmbeddedDataSpecification) -> Self {
        Class::EmbeddedDataSpecification(v)
    }
}
impl From<LevelType> for Class {
    fn from(v: LevelType) -> Self {
        Class::LevelType(v)
    }
}
impl From<ValueReferencePair> for Class {
    fn from(v: ValueReferencePair) -> Self {
        Class::ValueReferencePair(v)
    }
}
impl From<ValueList> for Class {
    fn from(v: ValueList) -> Self {
        Class::ValueList(v)
    }
}
impl From<LangStringPreferredNameTypeIec61360> for Class {
    fn from(v: LangStringPreferredNameTypeIec61360) -> Self {
        Class::LangStringPreferredNameTypeIec61360(v)
    }
}
impl From<LangStringShortNameTypeIec61360> for Class {
    fn from(v: LangStringShortNameTypeIec61360) -> Self {
        Class::LangStringShortNameTypeIec61360(v)
    }
}
impl From<LangStringDefinitionTypeIec61360> for Class {
    fn from(v: LangStringDefinitionTypeIec61360) -> Self {
        Class::LangStringDefinitionTypeIec61360(v)
    }
}
impl From<DataSpecificationIec61360> for Class {
    fn from(v: DataSpecificationIec61360) -> Self {
        Class::DataSpecificationIec61360(v)
    }
}

impl Class {
    /// Returns the [`ModelType`] of this class instance.
    pub fn model_type(&self) -> ModelType {
        match self {
            Class::Extension(_) => ModelType::Extension,
            Class::AdministrativeInformation(_) => ModelType::AdministrativeInformation,
            Class::Qualifier(_) => ModelType::Qualifier,
            Class::AssetAdministrationShell(_) => ModelType::AssetAdministrationShell,
            Class::AssetInformation(_) => ModelType::AssetInformation,
            Class::Resource(_) => ModelType::Resource,
            Class::SpecificAssetId(_) => ModelType::SpecificAssetId,
            Class::Submodel(_) => ModelType::Submodel,
            Class::RelationshipElement(_) => ModelType::RelationshipElement,
            Class::SubmodelElementList(_) => ModelType::SubmodelElementList,
            Class::SubmodelElementCollection(_) => ModelType::SubmodelElementCollection,
            Class::Property(_) => ModelType::Property,
            Class::MultiLanguageProperty(_) => ModelType::MultiLanguageProperty,
            Class::Range(_) => ModelType::Range,
            Class::ReferenceElement(_) => ModelType::ReferenceElement,
            Class::Blob(_) => ModelType::Blob,
            Class::File(_) => ModelType::File,
            Class::AnnotatedRelationshipElement(_) => ModelType::AnnotatedRelationshipElement,
            Class::Entity(_) => ModelType::Entity,
            Class::EventPayload(_) => ModelType::EventPayload,
            Class::BasicEventElement(_) => ModelType::BasicEventElement,
            Class::Operation(_) => ModelType::Operation,
            Class::OperationVariable(_) => ModelType::OperationVariable,
            Class::Capability(_) => ModelType::Capability,
            Class::ConceptDescription(_) => ModelType::ConceptDescription,
            Class::Reference(_) => ModelType::Reference,
            Class::Key(_) => ModelType::Key,
            Class::LangStringNameType(_) => ModelType::LangStringNameType,
            Class::LangStringTextType(_) => ModelType::LangStringTextType,
            Class::Environment(_) => ModelType::Environment,
            Class::EmbeddedDataSpecification(_) => ModelType::EmbeddedDataSpecification,
            Class::LevelType(_) => ModelType::LevelType,
            Class::ValueReferencePair(_) => ModelType::ValueReferencePair,
            Class::ValueList(_) => ModelType::ValueList,
            Class::LangStringPreferredNameTypeIec61360(_) => {
                ModelType::LangStringPreferredNameTypeIec61360
            }
            Class::LangStringShortNameTypeIec61360(_) => ModelType::LangStringShortNameTypeIec61360,
            Class::LangStringDefinitionTypeIec61360(_) => {
                ModelType::LangStringDefinitionTypeIec61360
            }
            Class::DataSpecificationIec61360(_) => ModelType::DataSpecificationIec61360,
        }
    }

    fn collect_descend_once(&self) -> Vec<Class> {
        match self {
            Class::Extension(x) => descend_extension(x),
            Class::AdministrativeInformation(x) => descend_administrative_information(x),
            Class::Qualifier(x) => descend_qualifier(x),
            Class::AssetAdministrationShell(x) => descend_asset_administration_shell(x),
            Class::AssetInformation(x) => descend_asset_information(x),
            Class::Resource(_) | Class::Key(_) | Class::LangStringNameType(_)
            | Class::LangStringTextType(_) | Class::LevelType(_)
            | Class::LangStringPreferredNameTypeIec61360(_)
            | Class::LangStringShortNameTypeIec61360(_)
            | Class::LangStringDefinitionTypeIec61360(_) => vec![],
            Class::SpecificAssetId(x) => descend_specific_asset_id(x),
            Class::Submodel(x) => descend_submodel(x),
            Class::RelationshipElement(x) => {
                let mut v = collect_sme_common_vec(x);
                v.push(Class::Reference(x.first.clone()));
                v.push(Class::Reference(x.second.clone()));
                v
            }
            Class::SubmodelElementList(x) => descend_submodel_element_list(x),
            Class::SubmodelElementCollection(x) => {
                let mut v = collect_sme_common_vec(x);
                if let Some(vals) = &x.value { v.extend(vals.iter().cloned()); }
                v
            }
            Class::Property(x) => {
                let mut v = collect_sme_common_vec(x);
                if let Some(vi) = &x.value_id { v.push(Class::Reference(vi.clone())); }
                v
            }
            Class::MultiLanguageProperty(x) => descend_multi_language_property(x),
            Class::Range(x) => collect_sme_common_vec(x),
            Class::ReferenceElement(x) => {
                let mut v = collect_sme_common_vec(x);
                if let Some(val) = &x.value { v.push(Class::Reference(val.clone())); }
                v
            }
            Class::Blob(x) => collect_sme_common_vec(x),
            Class::File(x) => collect_sme_common_vec(x),
            Class::AnnotatedRelationshipElement(x) => descend_annotated_relationship(x),
            Class::Entity(x) => descend_entity(x),
            Class::EventPayload(x) => descend_event_payload(x),
            Class::BasicEventElement(x) => {
                let mut v = collect_sme_common_vec(x);
                v.push(Class::Reference(x.observed.clone()));
                if let Some(mb) = &x.message_broker { v.push(Class::Reference(mb.clone())); }
                v
            }
            Class::Operation(x) => descend_operation(x),
            Class::OperationVariable(x) => vec![*x.value.clone()],
            Class::Capability(x) => collect_sme_common_vec(x),
            Class::ConceptDescription(x) => descend_concept_description(x),
            Class::Reference(x) => {
                let mut v = Vec::new();
                if let Some(rsi) = &x.referred_semantic_id {
                    v.push(Class::Reference(*rsi.clone()));
                }
                v.extend(x.keys.iter().map(|k| Class::Key(k.clone())));
                v
            }
            Class::Environment(x) => descend_environment(x),
            Class::EmbeddedDataSpecification(x) => {
                vec![
                    Class::Reference(x.data_specification.clone()),
                    *x.data_specification_content.clone(),
                ]
            }
            Class::ValueReferencePair(x) => vec![Class::Reference(x.value_id.clone())],
            Class::ValueList(x) => x
                .value_reference_pairs
                .iter()
                .map(|p| Class::ValueReferencePair(p.clone()))
                .collect(),
            Class::DataSpecificationIec61360(x) => descend_data_specification_iec61360(x),
        }
    }

    /// Returns an iterator over the direct children of this class instance.
    pub fn descend_once(&self) -> impl Iterator<Item = Class> + '_ {
        self.collect_descend_once().into_iter()
    }

    /// Returns an iterator over all descendants of this class instance in pre-order DFS.
    pub fn descend(&self) -> impl Iterator<Item = Class> + '_ {
        fn collect_descend(node: &Class, result: &mut Vec<Class>) {
            let children: Vec<Class> = node.collect_descend_once();
            for child in children {
                result.push(child.clone());
                collect_descend(&child, result);
            }
        }
        let mut result = Vec::new();
        collect_descend(self, &mut result);
        result.into_iter()
    }
}

/// Trait for submodel element types that share common fields.
trait SmeCommon {
    fn extensions(&self) -> &Option<Vec<Extension>>;
    fn display_name(&self) -> &Option<Vec<LangStringNameType>>;
    fn description(&self) -> &Option<Vec<LangStringTextType>>;
    fn semantic_id(&self) -> &Option<Reference>;
    fn supplemental_semantic_ids(&self) -> &Option<Vec<Reference>>;
    fn qualifiers(&self) -> &Option<Vec<Qualifier>>;
    fn embedded_data_specifications(&self) -> &Option<Vec<EmbeddedDataSpecification>>;
}

macro_rules! impl_sme_common {
    ($t:ty) => {
        impl SmeCommon for $t {
            fn extensions(&self) -> &Option<Vec<Extension>> { &self.extensions }
            fn display_name(&self) -> &Option<Vec<LangStringNameType>> { &self.display_name }
            fn description(&self) -> &Option<Vec<LangStringTextType>> { &self.description }
            fn semantic_id(&self) -> &Option<Reference> { &self.semantic_id }
            fn supplemental_semantic_ids(&self) -> &Option<Vec<Reference>> {
                &self.supplemental_semantic_ids
            }
            fn qualifiers(&self) -> &Option<Vec<Qualifier>> { &self.qualifiers }
            fn embedded_data_specifications(&self) -> &Option<Vec<EmbeddedDataSpecification>> {
                &self.embedded_data_specifications
            }
        }
    };
}

impl_sme_common!(RelationshipElement);
impl_sme_common!(SubmodelElementList);
impl_sme_common!(SubmodelElementCollection);
impl_sme_common!(Property);
impl_sme_common!(MultiLanguageProperty);
impl_sme_common!(Range);
impl_sme_common!(ReferenceElement);
impl_sme_common!(Blob);
impl_sme_common!(File);
impl_sme_common!(AnnotatedRelationshipElement);
impl_sme_common!(Entity);
impl_sme_common!(BasicEventElement);
impl_sme_common!(Operation);
impl_sme_common!(Capability);

fn collect_sme_common_vec(x: &impl SmeCommon) -> Vec<Class> {
    let mut v = Vec::new();
    if let Some(exts) = x.extensions() {
        v.extend(exts.iter().map(|e| Class::Extension(e.clone())));
    }
    if let Some(dn) = x.display_name() {
        v.extend(dn.iter().map(|l| Class::LangStringNameType(l.clone())));
    }
    if let Some(desc) = x.description() {
        v.extend(desc.iter().map(|l| Class::LangStringTextType(l.clone())));
    }
    if let Some(r) = x.semantic_id() {
        v.push(Class::Reference(r.clone()));
    }
    if let Some(refs) = x.supplemental_semantic_ids() {
        v.extend(refs.iter().map(|r| Class::Reference(r.clone())));
    }
    if let Some(qs) = x.qualifiers() {
        v.extend(qs.iter().map(|q| Class::Qualifier(q.clone())));
    }
    if let Some(eds) = x.embedded_data_specifications() {
        v.extend(eds.iter().map(|e| Class::EmbeddedDataSpecification(e.clone())));
    }
    v
}

fn descend_extension(x: &Extension) -> Vec<Class> {
    let mut v = Vec::new();
    if let Some(r) = &x.semantic_id { v.push(Class::Reference(r.clone())); }
    if let Some(refs) = &x.supplemental_semantic_ids {
        v.extend(refs.iter().map(|r| Class::Reference(r.clone())));
    }
    if let Some(refers) = &x.refers_to {
        v.extend(refers.iter().map(|r| Class::Reference(r.clone())));
    }
    v
}

fn descend_administrative_information(x: &AdministrativeInformation) -> Vec<Class> {
    let mut v = Vec::new();
    if let Some(eds) = &x.embedded_data_specifications {
        v.extend(eds.iter().map(|e| Class::EmbeddedDataSpecification(e.clone())));
    }
    if let Some(c) = &x.creator { v.push(Class::Reference(c.clone())); }
    v
}

fn descend_qualifier(x: &Qualifier) -> Vec<Class> {
    let mut v = Vec::new();
    if let Some(r) = &x.semantic_id { v.push(Class::Reference(r.clone())); }
    if let Some(refs) = &x.supplemental_semantic_ids {
        v.extend(refs.iter().map(|r| Class::Reference(r.clone())));
    }
    if let Some(vi) = &x.value_id { v.push(Class::Reference(vi.clone())); }
    v
}

fn descend_asset_administration_shell(x: &AssetAdministrationShell) -> Vec<Class> {
    let mut v = Vec::new();
    if let Some(exts) = &x.extensions { v.extend(exts.iter().map(|e| Class::Extension(e.clone()))); }
    if let Some(dn) = &x.display_name { v.extend(dn.iter().map(|l| Class::LangStringNameType(l.clone()))); }
    if let Some(desc) = &x.description { v.extend(desc.iter().map(|l| Class::LangStringTextType(l.clone()))); }
    if let Some(adm) = &x.administration { v.push(Class::AdministrativeInformation(adm.clone())); }
    if let Some(eds) = &x.embedded_data_specifications {
        v.extend(eds.iter().map(|e| Class::EmbeddedDataSpecification(e.clone())));
    }
    if let Some(df) = &x.derived_from { v.push(Class::Reference(df.clone())); }
    v.push(Class::AssetInformation(x.asset_information.clone()));
    if let Some(sms) = &x.submodels { v.extend(sms.iter().map(|r| Class::Reference(r.clone()))); }
    v
}

fn descend_asset_information(x: &AssetInformation) -> Vec<Class> {
    let mut v = Vec::new();
    if let Some(ids) = &x.specific_asset_ids {
        v.extend(ids.iter().map(|s| Class::SpecificAssetId(s.clone())));
    }
    if let Some(thumb) = &x.default_thumbnail { v.push(Class::Resource(thumb.clone())); }
    v
}

fn descend_specific_asset_id(x: &SpecificAssetId) -> Vec<Class> {
    let mut v = Vec::new();
    if let Some(r) = &x.semantic_id { v.push(Class::Reference(r.clone())); }
    if let Some(refs) = &x.supplemental_semantic_ids {
        v.extend(refs.iter().map(|r| Class::Reference(r.clone())));
    }
    if let Some(ext) = &x.external_subject_id { v.push(Class::Reference(ext.clone())); }
    v
}

fn descend_submodel(x: &Submodel) -> Vec<Class> {
    let mut v = Vec::new();
    if let Some(exts) = &x.extensions { v.extend(exts.iter().map(|e| Class::Extension(e.clone()))); }
    if let Some(dn) = &x.display_name { v.extend(dn.iter().map(|l| Class::LangStringNameType(l.clone()))); }
    if let Some(desc) = &x.description { v.extend(desc.iter().map(|l| Class::LangStringTextType(l.clone()))); }
    if let Some(adm) = &x.administration { v.push(Class::AdministrativeInformation(adm.clone())); }
    if let Some(r) = &x.semantic_id { v.push(Class::Reference(r.clone())); }
    if let Some(refs) = &x.supplemental_semantic_ids {
        v.extend(refs.iter().map(|r| Class::Reference(r.clone())));
    }
    if let Some(qs) = &x.qualifiers { v.extend(qs.iter().map(|q| Class::Qualifier(q.clone()))); }
    if let Some(eds) = &x.embedded_data_specifications {
        v.extend(eds.iter().map(|e| Class::EmbeddedDataSpecification(e.clone())));
    }
    if let Some(sme) = &x.submodel_elements { v.extend(sme.iter().cloned()); }
    v
}

fn descend_submodel_element_list(x: &SubmodelElementList) -> Vec<Class> {
    let mut v = collect_sme_common_vec(x);
    if let Some(sile) = &x.semantic_id_list_element { v.push(Class::Reference(sile.clone())); }
    if let Some(vals) = &x.value { v.extend(vals.iter().cloned()); }
    v
}

fn descend_multi_language_property(x: &MultiLanguageProperty) -> Vec<Class> {
    let mut v = collect_sme_common_vec(x);
    if let Some(vals) = &x.value { v.extend(vals.iter().map(|l| Class::LangStringTextType(l.clone()))); }
    if let Some(vi) = &x.value_id { v.push(Class::Reference(vi.clone())); }
    v
}

fn descend_annotated_relationship(x: &AnnotatedRelationshipElement) -> Vec<Class> {
    let mut v = collect_sme_common_vec(x);
    v.push(Class::Reference(x.first.clone()));
    v.push(Class::Reference(x.second.clone()));
    if let Some(anns) = &x.annotations { v.extend(anns.iter().cloned()); }
    v
}

fn descend_entity(x: &Entity) -> Vec<Class> {
    let mut v = collect_sme_common_vec(x);
    if let Some(stmts) = &x.statements { v.extend(stmts.iter().cloned()); }
    if let Some(ids) = &x.specific_asset_ids {
        v.extend(ids.iter().map(|s| Class::SpecificAssetId(s.clone())));
    }
    v
}

fn descend_event_payload(x: &EventPayload) -> Vec<Class> {
    let mut v = Vec::new();
    v.push(Class::Reference(x.source.clone()));
    if let Some(r) = &x.source_semantic_id { v.push(Class::Reference(r.clone())); }
    v.push(Class::Reference(x.observable_reference.clone()));
    if let Some(r) = &x.observable_semantic_id { v.push(Class::Reference(r.clone())); }
    if let Some(r) = &x.subject_id { v.push(Class::Reference(r.clone())); }
    v
}

fn descend_operation(x: &Operation) -> Vec<Class> {
    let mut v = collect_sme_common_vec(x);
    if let Some(ivs) = &x.input_variables { v.extend(ivs.iter().map(|ov| Class::OperationVariable(ov.clone()))); }
    if let Some(ovs) = &x.output_variables { v.extend(ovs.iter().map(|ov| Class::OperationVariable(ov.clone()))); }
    if let Some(iovs) = &x.inoutput_variables { v.extend(iovs.iter().map(|ov| Class::OperationVariable(ov.clone()))); }
    v
}

fn descend_concept_description(x: &ConceptDescription) -> Vec<Class> {
    let mut v = Vec::new();
    if let Some(exts) = &x.extensions { v.extend(exts.iter().map(|e| Class::Extension(e.clone()))); }
    if let Some(dn) = &x.display_name { v.extend(dn.iter().map(|l| Class::LangStringNameType(l.clone()))); }
    if let Some(desc) = &x.description { v.extend(desc.iter().map(|l| Class::LangStringTextType(l.clone()))); }
    if let Some(adm) = &x.administration { v.push(Class::AdministrativeInformation(adm.clone())); }
    if let Some(eds) = &x.embedded_data_specifications {
        v.extend(eds.iter().map(|e| Class::EmbeddedDataSpecification(e.clone())));
    }
    if let Some(ico) = &x.is_case_of { v.extend(ico.iter().map(|r| Class::Reference(r.clone()))); }
    v
}

fn descend_environment(x: &Environment) -> Vec<Class> {
    let mut v = Vec::new();
    if let Some(aas) = &x.asset_administration_shells {
        v.extend(aas.iter().map(|a| Class::AssetAdministrationShell(a.clone())));
    }
    if let Some(sms) = &x.submodels { v.extend(sms.iter().map(|s| Class::Submodel(s.clone()))); }
    if let Some(cds) = &x.concept_descriptions { v.extend(cds.iter().map(|c| Class::ConceptDescription(c.clone()))); }
    v
}

fn descend_data_specification_iec61360(x: &DataSpecificationIec61360) -> Vec<Class> {
    let mut v = Vec::new();
    v.extend(x.preferred_name.iter().map(|l| Class::LangStringPreferredNameTypeIec61360(l.clone())));
    if let Some(sn) = &x.short_name {
        v.extend(sn.iter().map(|l| Class::LangStringShortNameTypeIec61360(l.clone())));
    }
    if let Some(uid) = &x.unit_id { v.push(Class::Reference(uid.clone())); }
    if let Some(def) = &x.definition {
        v.extend(def.iter().map(|l| Class::LangStringDefinitionTypeIec61360(l.clone())));
    }
    if let Some(vl) = &x.value_list { v.push(Class::ValueList(vl.clone())); }
    if let Some(lt) = &x.level_type { v.push(Class::LevelType(lt.clone())); }
    v
}
