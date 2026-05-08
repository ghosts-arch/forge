import type { ModelRepository } from "$lib/server/infrastructure/model/model.repository";

export class ModelService {
	constructor(private readonly modelRepository: ModelRepository) {
		this.modelRepository = modelRepository;
	}

	getModels = async () => {
		const models = await this.modelRepository.getModels();
		return models;
	};

	deleteModel = async (uuid: string) => {
		await this.modelRepository.deleteModel(uuid);
	};
}
