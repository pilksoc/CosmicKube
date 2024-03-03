use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::kube::Kube;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
	id: Uuid,
	output_id: Uuid,
	output_kube: Kube,
	kube1_id: Uuid,
	kube2_id: Uuid,
}