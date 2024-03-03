use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::kube::{Kube, KubeId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
	id: Uuid,
	outputId: Uuid,
	outputKube: Kube,
	kube1_id: KubeId,
	kube2_id: KubeId,
}