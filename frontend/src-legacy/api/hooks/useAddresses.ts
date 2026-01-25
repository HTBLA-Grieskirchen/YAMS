import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { useBackend } from "../useBackend";
import { Address } from "../client";

export const useAddresses = () => {
  const backend = useBackend();
  const queryClient = useQueryClient();

  const query = useQuery({
    queryKey: ["addresses"],
    queryFn: () => backend.getAddresses(),
  });

  const createMutation = useMutation({
    mutationFn: (address: Address) => backend.createAddress(address),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["addresses"] });
    },
  });

  return {
    addresses: query.data ?? [],
    isLoading: query.isLoading,
    isError: query.isError,
    createAddress: createMutation.mutateAsync,
  };
};
